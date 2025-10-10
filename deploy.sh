
#!/usr/bin/env bash
# Manual CI/CD: build locally, ship only images + required runtime configs, run on server.
# Local req: docker. Server req: docker compose.
set -euo pipefail

### CONFIG
SSH_USER="root"
SSH_HOST="176.126.87.212"
SSH_PORT="22"
REMOTE_DIR="/root/development/luckywin-back"

# Base compose on server (kept under version control)
COMPOSE_FILE="docker-compose.yml"

# WHITELIST: ONLY these paths get synced to server (relative to repo root)
# Include runtime-only config, not secrets or volumes.
SYNC_ITEMS=(
  "docker-compose.yml"           # compose file
  "./migration/"                    # DB migrations crate (SeaORM)
	"Makefile"                      # Makefile
  #"ops/haproxy-smtp.cfg"              # smtp-proxy haproxy cfg
)

# Build contexts/targets
API_CONTEXT="."
API_TARGET="prod"
# MAILER_CONTEXT="./mailer"
# MAILER_TARGET="prod"

# Image names
IMG_API="luckywin_api"
# IMG_MAILER="luckywin_mailer"

# Tag
if git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  TAG="$(git rev-parse --short HEAD)"
else
  TAG="$(date -u +%Y%m%d%H%M%S)"
fi
IMG_TAG_API="${IMG_API}:${TAG}"
# IMG_TAG_MAILER="${IMG_MAILER}:${TAG}"
TAR_FILE="/tmp/luckywin_images_${TAG}.tar"
OVERRIDE_NAME="docker-compose.localimage.${TAG}.yml"
OVERRIDE_REMOTE="${REMOTE_DIR}/${OVERRIDE_NAME}"

SSH="ssh -p ${SSH_PORT} ${SSH_USER}@${SSH_HOST}"
SCP="scp -P ${SSH_PORT}"

### SSH AGENT / SINGLE PASSPHRASE PROMPT (runtime only)
# Goal: ask for SSH key passphrase once, use agent for all SSH/SCP/RSYNC calls.
# Nothing is persisted; passphrase kept in-memory only and purged on exit.

AGENT_STARTED=0
ASKPASS=""

cleanup() {
  # Remove temp askpass and unset passphrase
  if [ -n "${ASKPASS}" ] && [ -f "${ASKPASS}" ]; then rm -f "${ASKPASS}" || true; fi
  unset SSH_PASSPHRASE || true
  # Stop agent only if we started it here
  if [ "${AGENT_STARTED}" = "1" ]; then eval "$(ssh-agent -k)" >/dev/null 2>&1 || true; fi
}
trap cleanup EXIT

# Start agent if not present
if [ -z "${SSH_AUTH_SOCK:-}" ]; then
  eval "$(ssh-agent -s)"
  AGENT_STARTED=1
fi

# Determine default key path
KEY_PATH="${HOME}/.ssh/id_ed25519"
if [ ! -f "${KEY_PATH}" ]; then
  KEY_PATH="${HOME}/.ssh/id_rsa"
fi
if [ ! -f "${KEY_PATH}" ]; then
  echo "ERROR: No default SSH key found (~/.ssh/id_ed25519 or ~/.ssh/id_rsa)." >&2
  echo "Please set KEY_PATH env or create a key." >&2
  exit 1
fi

echo "==> Adding SSH key to agent: ${KEY_PATH}"
if ssh-add -l >/dev/null 2>&1; then
  # Try to detect if the key is already loaded
  if ssh-add -l | grep -q "$(ssh-keygen -lf "${KEY_PATH}" | awk '{print $2}')"; then
    echo "    Key already loaded in agent."
  else
    # Prompt once for passphrase; use askpass helper if provided
    read -r -s -p "SSH key passphrase (leave empty to type interactively): " SSH_PASSPHRASE || true
    echo ""
    if [ -n "${SSH_PASSPHRASE}" ]; then
      ASKPASS="$(mktemp)"
      chmod 700 "${ASKPASS}"
      cat >"${ASKPASS}" <<'EOF'
#!/usr/bin/env bash
exec bash -lc 'printf %s "${SSH_PASSPHRASE}"'
EOF
      # Force askpass (detach from TTY)
      SSH_ASKPASS="${ASKPASS}" SSH_ASKPASS_REQUIRE=force setsid -w ssh-add "${KEY_PATH}" </dev/null
    else
      ssh-add "${KEY_PATH}"
    fi
  fi
else
  # Fallback: just run ssh-add (older clients)
  ssh-add "${KEY_PATH}"
fi

### TESTS (fail-fast)
# echo "==> Bringing down local dev containers"
# docker compose -f docker-compose.yml --profile dev down || true
#
# echo "==> Cleaning local build artifacts (api/, mailer/)"
# sudo rm -rf api/target mailer/target || true

### TESTS (fail-fast)
echo "==> Running API tests"
( cargo test ) || { echo "ERROR: API tests failed. Aborting deploy."; exit 1; }

# echo "==> Running Mailer tests"
# ( cd mailer && cargo test ) || { echo "ERROR: Mailer tests failed. Aborting deploy."; exit 1; }
#
### BUILD
echo "==> Build images (tag ${TAG})"
docker build --target "${API_TARGET}"    -t "${IMG_TAG_API}"     "${API_CONTEXT}"
# docker build --target "${MAILER_TARGET}" -t "${IMG_TAG_MAILER}"  "${MAILER_CONTEXT}"

echo "==> Save images to ${TAR_FILE}"
docker save -o "${TAR_FILE}" "${IMG_TAG_API}"

### SYNC WHITELISTED CONFIG ONLY
echo "==> Ensure remote dir"
${SSH} "mkdir -p '${REMOTE_DIR}'"

echo "==> Sync whitelisted runtime config"
for item in "${SYNC_ITEMS[@]}"; do
  # Create parent dir on server
  REMOTE_PARENT="${REMOTE_DIR}/$(dirname "${item}")"
  ${SSH} "mkdir -p '${REMOTE_PARENT}'"
  # Sync item (file or directory) - exclude build artifacts and data directories
  rsync -az -e "ssh -p ${SSH_PORT}" --delete \
    --exclude='target' \
    --exclude='luckywin_db_data' \
    --exclude='licky_win_db_data' \
    "${item}" "${SSH_USER}@${SSH_HOST}:${REMOTE_DIR}/${item%/}"
done

### SHIP IMAGES
echo "==> Upload images tar"
${SCP} "${TAR_FILE}" "${SSH_USER}@${SSH_HOST}:${REMOTE_DIR}/"

### OVERRIDE COMPOSE: pin images, disable build (others unchanged)
echo "==> Create compose override on server"
${SSH} "cat > '${OVERRIDE_REMOTE}' <<'YAML'
services:
  api-prod:
    image: ${IMG_TAG_API}
    build: null
    environment:
      RUST_LOG: info
YAML
"

### DEPLOY
echo "==> Load images + compose up"
${SSH} "bash -lc \"
  set -euo pipefail
  cd '${REMOTE_DIR}'
  docker load -i '$(basename "${TAR_FILE}")'
  docker compose -f '${COMPOSE_FILE}' -f '${OVERRIDE_NAME}' --profile prod up -d
  echo '==> Checking for gateway service'
  if docker ps -a --format '{{.Names}}' | grep -q '^luckywin_gateway$'; then
    echo '==> Restarting luckywin_gateway to load updated routes'
    docker restart luckywin_gateway
  else
    echo '==> No gateway service found, skipping gateway restart'
  fi
  echo '==> Running database migrations and seed'
  # Ensure Rust/Cargo available in non-interactive shell
  if [ -f "\$HOME/.cargo/env" ]; then
    . "\$HOME/.cargo/env"
  else
    export PATH="\$HOME/.cargo/bin:\$PATH"
  fi
  command -v cargo >/dev/null 2>&1 || { echo 'cargo still not found; PATH='"\$PATH"; exit 127; }
  cd '${REMOTE_DIR}'
  make migrate-up
  make seed
\""

### CLEAN
echo "==> Clean local tar"
rm -f "${TAR_FILE}"

echo "==> OK: deployed ${TAG}"
