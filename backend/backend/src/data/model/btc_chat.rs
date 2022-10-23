use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::data::schema::*;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Insertable, AsChangeset, Clone)]
#[changeset_options(treat_none_as_null = "true")]
#[table_name = "btc_chats"]
pub struct BtcChat {
    pub id: Uuid,
    pub active: bool,
    pub processing_status_code: ProcessingStatusValue,
    pub message: Option<String>,
    pub sender: Option<String>,
    pub store_id: String,
    pub invoice_id: String,
    pub amount_of_sats: i64,
    pub amount_in_fiat: i32,
    pub currency: String,
    pub date_created: DateTime<Utc>,
    pub date_modified: DateTime<Utc>,
}

impl BtcChat {
    pub fn new(id: Uuid,
               active: bool,
               processing_status_code: ProcessingStatusValue,
               message: Option<String>,
               sender: Option<String>,
               store_id: String,
               invoice_id: String,
               amount_of_sats: i64,
               amount_in_fiat: i32,
               currency: String,
               date_created: DateTime<Utc>,
               date_modified: DateTime<Utc>) -> Self {
        BtcChat {
            id,
            active,
            processing_status_code,
            message,
            sender,
            store_id,
            invoice_id,
            amount_of_sats,
            amount_in_fiat,
            currency,
            date_created,
            date_modified,
        }
    }

    pub fn processing_status(&self) -> ProcessingStatus {
        ProcessingStatus::from_code(self.processing_status_code)
    }
}

pub type ProcessingStatusValue = i32;

#[derive(PartialEq)]
pub enum ProcessingStatus {
    UnConfirmed,
    Discarded,
    Confirmed,
    ProcessedByCreator,
    Undefined
}

impl ProcessingStatus {
    pub fn from_code(code: ProcessingStatusValue) -> Self {
        match code {
            0 => ProcessingStatus::UnConfirmed,
            1 => ProcessingStatus::Discarded,
            2 => ProcessingStatus::Confirmed,
            3 => ProcessingStatus::ProcessedByCreator,
            _ => ProcessingStatus::Undefined,
        }
    }

    pub fn to_code(&self) -> ProcessingStatusValue {
        match *self {
            ProcessingStatus::UnConfirmed => 0,
            ProcessingStatus::Discarded => 1,
            ProcessingStatus::Confirmed => 2,
            ProcessingStatus::ProcessedByCreator => 3,
            ProcessingStatus::Undefined => -1,
        }
    }
}

