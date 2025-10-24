#!/bin/bash
# PostgreSQL entrypoint that reads credentials from Docker secrets
set -e

# Read credentials from secrets
if [ -f /run/secrets/db_user ]; then
  export POSTGRES_USER=$(cat /run/secrets/db_user)
fi

if [ -f /run/secrets/db_password ]; then
  export POSTGRES_PASSWORD=$(cat /run/secrets/db_password)
fi

if [ -f /run/secrets/db_name ]; then
  export POSTGRES_DB=$(cat /run/secrets/db_name)
fi

# Execute the original PostgreSQL entrypoint
exec docker-entrypoint.sh "$@"
