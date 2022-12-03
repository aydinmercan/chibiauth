/* API Tokens */
CREATE TABLE IF NOT EXISTS api_token
    ( selector    TEXT PRIMARY KEY
    , verifier    BLOB NOT NULL
    , expiration  INTEGER NOT NULL /* TAI64N */
    , description TEXT
    ) STRICT;

/* General User Management */
CREATE TABLE IF NOT EXISTS users
    ( uuid     BLOB PRIMARY KEY
    , username TEXT NOT NULL
    , passhash TEXT NOT NULL
    ) STRICT;

CREATE TABLE IF NOT EXISTS pass_reset
    ( selector   TEXT PRIMARY KEY
    , verifier   BLOB NOT NULL
    , expiration INTEGER NOT NULL
    , uuid       BLOB NOT NULL REFERENCES users ON DELETE CASCADE
    ) STRICT;

/* OAuth & OIDC */
CREATE TABLE IF NOT EXISTS oidc_rs256
    ( kid       TEXT UNIQUE NOT NULL
    , keyblob   BLOB NOT NULL
    ) STRICT;

CREATE TABLE IF NOT EXISTS oidc_hs256
    ( kid       TEXT UNIQUE NOT NULL
    , keyblob   BLOB NOT NULL
    ) STRICT;

CREATE TABLE IF NOT EXISTS oidc_generation
    ( tokentype  TEXT NOT NULL
    , not_before INTEGER NOT NULL
    ) STRICT;

CREATE TABLE IF NOT EXISTS oidc_clients
    ( uuid BLOB  PRIMARY KEY
    , name TEXT NOT NULL
    , subdomains TEXT NOT NULL
    , kid        TEXT NOT NULL
    , is_secret  INTEGER NOT NULL
    ) STRICT;

CREATE TABLE IF NOT EXISTS oidc_session_tokens
    ( tokenhash  BLOB PRIMARY KEY
    , uuid       BLOB REFERENCES users ON DELETE CASCADE,
    , generation INTEGER NOT NULL
    , expiration INTEGER NOT NULL
    ) STRICT;

CREATE TABLE IF NOT EXISTS oidc_refresh_tokens
    ( tokenhash  BLOB PRIMARY KEY
    , uuid       BLOB REFERENCES users ON DELETE CASCADE
    , generation INTEGER NOT NULL
    , expiration INTEGER NOT NULL
    ) STRICT;
