#!/bin/bash
set -e

if [ -z "${MORG_DB_NAME}" ]; then
  DATABASE="$(< /run/secrets/MORG_DB_NAME)"
else
  DATABASE="${MORG_DB_NAME}"
fi 

if [ -z "${MORG_DB_USER}" ]; then
  USER="$(< /run/secrets/MORG_DB_USER)"
else
  USER="${MORG_DB_USER}"
fi

if [ -z "${MORG_DB_PASS}" ]; then
  PASSW="$(< /run/secrets/MORG_DB_PASS)"
else
  PASSW="${MORG_DB_PASS}"
fi

READONLY=${READONLY_ROLE}
DATABASE_USER=${DATABASE_USER}

# down

PGPASSWORD="$POSTGRES_PASSWORD" psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" <<-EOSQL
	GRANT ALL ON DATABASE ${DATABASE} TO PUBLIC;

	REVOKE CONNECT ON DATABASE ${DATABASE} FROM ${DATABASE_USER};
	REVOKE ${DATABASE_USER} FROM ${READONLY};
	REVOKE ${DATABASE_USER} FROM ${USER};

	DROP DATABASE IF EXISTS ${DATABASE};
	DROP ROLE IF EXISTS ${DATABASE_USER};
	DROP USER IF EXISTS ${USER};
EOSQL

# up

PGPASSWORD="$POSTGRES_PASSWORD" psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" <<-EOSQL
	CREATE DATABASE ${DATABASE};
	CREATE ROLE	${DATABASE_USER};

	CREATE USER ${USER} WITH ENCRYPTED PASSWORD '${PASSW}';

	GRANT ${DATABASE_USER} TO ${USER};
	GRANT ${DATABASE_USER} TO ${READONLY};

	REVOKE ALL ON DATABASE ${DATABASE} FROM PUBLIC;
	GRANT CONNECT ON DATABASE ${DATABASE} TO ${DATABASE_USER};
EOSQL

PGPASSWORD="$POSTGRES_PASSWORD" psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" "${DATABASE}" <<-EOSQL
	REVOKE ALL ON SCHEMA public FROM PUBLIC;
	GRANT USAGE ON SCHEMA public TO ${DATABASE_USER};

	ALTER DEFAULT PRIVILEGES IN SCHEMA public
		GRANT USAGE, SELECT ON SEQUENCES TO ${DATABASE_USER};
	ALTER DEFAULT PRIVILEGES IN SCHEMA public
		GRANT SELECT ON TABLES TO ${READONLY};
	ALTER DEFAULT PRIVILEGES IN SCHEMA public
		GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO ${USER};
EOSQL