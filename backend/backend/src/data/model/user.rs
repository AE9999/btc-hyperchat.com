use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::data::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Insertable, AsChangeset, Clone)]
#[changeset_options(treat_none_as_null = "true")]
#[table_name = "users"]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub initial_btc_password: String,
    pub keycloak_id: Uuid,
    pub active: bool,
    pub date_created: DateTime<Utc>,
    pub date_modified: DateTime<Utc>,
}

impl User {

    pub fn new(id: Uuid,
               username: String,
               email:String,
               initial_btc_password: String,
               keycloak_id: Uuid,
               active: bool,
               date_created: DateTime<Utc>,
               date_modified: DateTime<Utc>) -> Self {
        User {
            id,
            username,
            email,
            initial_btc_password,
            keycloak_id,
            active,
            date_created,
            date_modified,
        }
    }

    pub fn pk(&self) -> Uuid {
        self.id.clone()
    }
}
