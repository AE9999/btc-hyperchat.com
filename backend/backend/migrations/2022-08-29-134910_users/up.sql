-- Your SQL goes here
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY
    ,username TEXT NOT NULL
    ,email TEXT NOT NULL
    ,initial_btc_password TEXT NOT NULL
    ,keycloak_id UUID NOT NULL UNIQUE
    ,active BOOLEAN NOT NULL
    ,date_created TIMESTAMPTZ NOT NULL
    ,date_modified TIMESTAMPTZ NOT NULL
);

CREATE INDEX "users_keycloak_id_x"
    ON users (keycloak_id);

CREATE INDEX "users_username_x"
    ON users (username);

CREATE INDEX index_users_on_username_trigram
    ON users
    USING gin (username gin_trgm_ops);
