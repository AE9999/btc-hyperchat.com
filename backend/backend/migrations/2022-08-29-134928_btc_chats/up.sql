-- Your SQL goes here
CREATE TABLE IF NOT EXISTS btc_chats (
    id UUID PRIMARY KEY
    ,active BOOLEAN NOT NULL
    ,processing_status_code INT NOT NULL
    ,message TEXT
    ,sender TEXT
    ,store_id TEXT NOT NULL
    ,invoice_id TEXT NOT NULL UNIQUE
    ,amount_of_sats BIGINT NOT NULL
    ,amount_in_fiat INT NOT NULL
    ,currency TEXT NOT NULL
    ,date_created TIMESTAMPTZ NOT NULL
    ,date_modified TIMESTAMPTZ NOT NULL
);
