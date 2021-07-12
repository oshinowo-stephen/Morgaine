#!/bin/bash
set -e

if [ -z "${READONLY_USER}" ]; then
  USER="$(< /run/secrets/READONLY_USER)"
else
  USER=${READONLY_USER}
fi

if [ -z "${READONLY_PASS}" ]; then
  PASSW="$(< /run/secrets/READONLY_PASS)"
else
  PASSW=${READONLY_PASS}
fi

READONLY=$READONLY_ROLE

PGPASSWORD="$POSTGRES_PASSWORD" psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" <<-EOSQL
  REVOKE ${READONLY} FROM ${USER};
  
  DROP ROLE IF EXISTS ${READONLY};
  DROP ROLE IF EXISTS ${USER};
EOSQL

# up

PGPASSWORD="$POSTGRES_PASSWORD" psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" <<-EOSQL
  CREATE ROLE ${READONLY};
  CREATE USER ${USER} WITH ENCRYPTED PASSWORD '${PASSW}';
	GRANT ${READONLY} TO ${USER};
EOSQL
