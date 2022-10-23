-- Your SQL goes here
CREATE TABLE IF NOT EXISTS stores (
    id UUID PRIMARY KEY
    ,owner_id UUID NOT NULL
    ,btcpay_store_id TEXT NOT NULL UNIQUE
    ,webhook_config_json TEXT NOT NULL
    ,webhook_active BOOLEAN NOT NULL
    ,automatically_process_btc_chats_if_webhook_succeeds BOOLEAN NOT NULL default FALSE
    ,active BOOLEAN NOT NULL
    ,date_created TIMESTAMPTZ NOT NULL
    ,date_modified TIMESTAMPTZ NOT NULL
    ,CONSTRAINT fk_stores_to_users
        FOREIGN KEY(owner_id)
            REFERENCES users(id)
);

CREATE INDEX "stores_btcpay_store_id_x"
    ON stores (btcpay_store_id);
