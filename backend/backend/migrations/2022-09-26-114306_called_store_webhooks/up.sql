-- Your SQL goes here
CREATE TABLE IF NOT EXISTS called_store_webhooks(
    id       UUID PRIMARY KEY
    ,store_id UUID NOT NULL
    ,btc_chat_id UUID NOT NULL
    ,called_webhook TEXT NOT NULL
    ,result_code INT NOT NULL
    ,date_created TIMESTAMPTZ NOT NULL
)
