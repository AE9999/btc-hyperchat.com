use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::data::schema::*;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Insertable, AsChangeset, Clone)]
#[changeset_options(treat_none_as_null = "true")]
#[table_name = "stores"]
pub struct Store {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub btcpay_store_id: String,
    pub webhook_config_json: String,
    pub webhook_active: bool,
    pub automatically_process_btc_chats_if_webhook_succeeds: bool,
    pub active: bool,
    pub date_created: DateTime<Utc>,
    pub date_modified: DateTime<Utc>,
}

impl Store {
    pub fn new(id: Uuid,
               owner_id: Uuid,
               btcpay_store_id: String,
               webhook_config_json: String,
               webhook_active: bool,
               automatically_process_btc_chats_if_webhook_succeeds: bool,
               active: bool,
               date_created: DateTime<Utc>,
               date_modified: DateTime<Utc>) -> Self {
        Store {
            id,
            owner_id,
            btcpay_store_id,
            webhook_config_json,
            webhook_active,
            automatically_process_btc_chats_if_webhook_succeeds,
            active,
            date_created,
            date_modified,
        }
    }

    pub fn pk(&self) -> Uuid {
        self.id.clone()
    }
}
