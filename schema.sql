CREATE TABLE IF NOT EXISTS users (
    uuid       BLOB PRIMARY KEY NOT NULL,
    username   TEXT NOT NULL UNIQUE,
    secrethash TEXT NOT NULL
) STRICT;

CREATE TABLE IF NOT EXISTS oidc_claims (
    uuid                  BLOB REFERENCES users(uuid) ON DELETE CASCADE,
    sub                   TEXT NOT NULL,
    name                  TEXT,
    given_name            TEXT,
    family_name           TEXT,
    middle_name           TEXT,
    nickname              TEXT,
    preferred_username    TEXT,
    profile               TEXT,
    picture               TEXT,
    website               TEXT,
    email                 TEXT,
    email_verified        INTEGER NOT NULL DEFAULT 'FALSE',
    gender                TEXT,
    birthdate             TEXT,
    zoneinfo              TEXT,
    locale                TEXT,
    phone_number          TEXT,
    phone_number_verified INTEGER NOT NULL DEFAULT 'FALSE',
    address               TEXT,
    updated_at            INTEGER
) STRICT;

CREATE TABLE IF NOT EXISTS sessions (
    cookie BLOB PRIMARY KEY NOT NULL,
    uuid   BLOB NOT NULL REFERENCES users(uuid) ON DELETE CASCADE
) STRICT;

CREATE TABLE IF NOT EXISTS reset_token (
    selector   TEXT PRIMARY KEY NOT NULL,
    uuid       BLOB NOT NULL REFERENCES users(uuid) ON DELETE CASCADE,
    verifier   TEXT NOT NULL
    expires_at TEXT NOT NULL,
) STRICT;
