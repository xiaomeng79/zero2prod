#!/usr/bin/env bash
set -x
set -eo pipefail

DB_HOST="${DB_HOST:=localhost}"
DB_USER=${DB_USER:=postgres}
DB_PASSWORD="${DB_PASSWORD:=password}"
PGPASSWORD="${DB_PASSWORD:=password}"
DB_NAME="${DB_NAME:=newsletter}"
DB_PORT="${DB_PORT:=5432}"

# Allow to skip Docker if a dockerized Postgres database is already running
if [[ -z "${SKIP_DOCKER}" ]]
then
  docker run \
      -e POSTGRES_USER=${DB_USER} \
      -e POSTGRES_PASSWORD=${DB_PASSWORD} \
      -e POSTGRES_DB=${DB_NAME} \
      -p "${DB_PORT}":5432 \
      -d postgres \
      postgres -N 1000
fi

until PGPASSWORD="${DB_PASSWORD}" psql -h ${DB_HOST} -U "${DB_USER}"  -p "${DB_PORT}" -d "postgres" -c '\q'; do
  >&2 echo "Postgres is still unavailable - sleeping"
  sleep 1
done

>&2 echo "Postgres is up and running on port ${DB_PORT} - running migrations now!"

export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}
echo "DATABASE_URL=${DATABASE_URL}" > .env
sqlx database create
sqlx migrate run

>&2 echo "Postgres has been migrated, ready to go!"