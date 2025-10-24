# Define the database URL here or expect it in your environment
# Try to read from secrets/dsn_seeder.txt first, fallback to default
DATABASE_URL ?= $(shell [ -f secrets/dsn_seeder.txt ] && cat secrets/dsn_seeder.txt || echo "postgres://postgres:postgres@localhost:5436/luckywin")

# Path to sea-orm-cli
SEA = sea-orm-cli

# Run all pending migrations
migrate-up:
	DATABASE_URL=$(DATABASE_URL) cargo run --manifest-path ./migration/Cargo.toml --bin migration -- up -u $(DATABASE_URL)

# Rollback the last migration
migrate-down:
	DATABASE_URL=$(DATABASE_URL) cargo run --manifest-path ./migration/Cargo.toml --bin migration -- down -u $(DATABASE_URL)

# Redo the last migration (down + up)
migrate-redo:
	DATABASE_URL=$(DATABASE_URL) $(SEA) migrate redo

# Refresh: drop all tables and rerun all migrations
migrate-refresh:
	DATABASE_URL=$(DATABASE_URL) cargo run --manifest-path ./migration/Cargo.toml --bin migration -- refresh -u $(DATABASE_URL)

# Generate a new migration (pass name like: make migrate-gen name=create_users)
migrate-gen:
	$(SEA) migrate generate $(name)

# Show current migration status
migrate-status:
	DATABASE_URL=$(DATABASE_URL) $(SEA) migrate status

generate-entities:
	@TMP_DIR=$$(mktemp -d); \
	  echo "Generating entities into $$TMP_DIR"; \
	  $(SEA) generate entity -u $(DATABASE_URL) -o $$TMP_DIR; \
	  echo "Syncing generated entities"; \
	  rsync -a --delete $$TMP_DIR/ ./src/domain/entities/; \
	  rm -rf $$TMP_DIR;
seed:
	DATABASE_URL=$(DATABASE_URL) cargo run --manifest-path ./migration/Cargo.toml --bin seed -- up -u $(DATABASE_URL)
