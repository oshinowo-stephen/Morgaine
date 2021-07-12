#!bin/bash
set -e

if [ -z "${MORG_DB_NAME}" ]; then
	DATABASE="$(< /run/secrets/MORG_DB_NAME)"
else
	DATABASE="${MORG_DB_NAME}"
fi

# DOWN

PGPASSWORD="$POSTGRES_PASSWORD" psql -v ON_ERROR_STOP=1 --dbname="morg" --username "$POSTGRES_USER" <<-EOSQL
	DROP TABLE IF EXISTS account_questions;
	DROP TABLE IF EXISTS passwords;
	DROP TABLE IF EXISTS accounts;
	DROP TABLE IF EXISTS vaults;
	DROP TABLE IF EXISTS masters;
EOSQL

# UP

PGPASSWORD="$POSTGRES_PASSWORD" psql -v ON_ERROR_STOP=1 --dbname="morg" --username "$POSTGRES_USER" <<-EOSQL
	CREATE TABLE IF NOT EXISTS masters (
		master_passwd VARCHAR NOT NULL,
		created_at VARCHAR NOT NULL,
		name VARCHAR UNIQUE NOT NULL,
		id VARCHAR NOT NULL PRIMARY KEY
	);

	CREATE TABLE IF NOT EXISTS vaults (
		master_id VARCHAR NOT NULL,
		created_at VARCHAR NOT NULL,
		title VARCHAR NOT NULL UNIQUE,
		id VARCHAR NOT NULL PRIMARY KEY,
		FOREIGN KEY (master_id) REFERENCES masters(id)
	);

	CREATE TABLE IF NOT EXISTS passwords (
		vault_id VARCHAR NOT NULL,
		account_id VARCHAR NOT NULL,
		vault_passwd VARCHAR NOT NULL,
		id VARCHAR NOT NULL PRIMARY KEY
	);

	CREATE TABLE IF NOT EXISTS accounts (
		phone VARCHAR,
		username VARCHAR,
		email VARCHAR NOT NULL,
		domain VARCHAR NOT NULL,
		id VARCHAR NOT NULL PRIMARY KEY
	);

	CREATE TABLE IF NOT EXISTS account_questions (
		account_id VARCHAR NOT NULL,
		question_type VARCHAR NOT NULL,
		question_name VARCHAR NOT NULL,
		question_answ VARCHAR NOT NULL,
		id VARCHAR NOT NULL PRIMARY KEY,
		FOREIGN KEY (account_id) REFERENCES accounts(id)
	);
EOSQL