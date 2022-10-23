// @generated automatically by Diesel CLI.

diesel::table! {
    btc_chats (id) {
        id -> Uuid,
        active -> Bool,
        processing_status_code -> Int4,
        message -> Nullable<Text>,
        sender -> Nullable<Text>,
        store_id -> Text,
        invoice_id -> Text,
        amount_of_sats -> Int8,
        amount_in_fiat -> Int4,
        currency -> Text,
        date_created -> Timestamptz,
        date_modified -> Timestamptz,
    }
}

diesel::table! {
    called_store_webhooks (id) {
        id -> Uuid,
        store_id -> Uuid,
        btc_chat_id -> Uuid,
        called_webhook -> Text,
        result_code -> Int4,
        date_created -> Timestamptz,
    }
}

diesel::table! {
    stores (id) {
        id -> Uuid,
        owner_id -> Uuid,
        btcpay_store_id -> Text,
        webhook_config_json -> Text,
        webhook_active -> Bool,
        automatically_process_btc_chats_if_webhook_succeeds -> Bool,
        active -> Bool,
        date_created -> Timestamptz,
        date_modified -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Text,
        email -> Text,
        initial_btc_password -> Text,
        keycloak_id -> Uuid,
        active -> Bool,
        date_created -> Timestamptz,
        date_modified -> Timestamptz,
    }
}

diesel::joinable!(stores -> users (owner_id));

diesel::allow_tables_to_appear_in_same_query!(
    btc_chats,
    called_store_webhooks,
    stores,
    users,
);
