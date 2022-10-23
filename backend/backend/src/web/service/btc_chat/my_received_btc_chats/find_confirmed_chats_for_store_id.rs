use actix_web::http::StatusCode;
use std::fmt;
use actix_web::web::Data;
use crate::AppContext;
use crate::data::model::btc_chat::{BtcChat, ProcessingStatus};


pub fn find_new_btc_chats_for_store_id(store_id: &str,
                                       context: &Data<AppContext>) -> Result<Vec<BtcChat>,
                                                                          (StatusCode, String)> {
    log::info!("{:?} {:?}",
                LogEvents::FindConfirmedBtcChatsForStoreStart,
                store_id);

    let btc_chats =
        context.repositories
            .btc_chat_repository
            .find_by_store_id_and_processing_status_code_order_by_date_created_asc(store_id,
                                                                &ProcessingStatus::Confirmed.to_code());

    if btc_chats.is_err() {
        log::error!("{:?} {:?}",
                    LogEvents::FindConfirmedBtcChatsForStoreError,
                    btc_chats.as_ref().err().unwrap().to_string());
        return Err((StatusCode::INTERNAL_SERVER_ERROR,
                    LogEvents::FindConfirmedBtcChatsForStoreError.to_string()))
    }

    let btc_chats = btc_chats.unwrap();

    log::info!("{:?}",
                LogEvents::FindConfirmedBtcChatsForStoreOk);

    Ok(btc_chats)
}

#[derive(Debug)]
enum LogEvents {
    FindConfirmedBtcChatsForStoreStart,
    FindConfirmedBtcChatsForStoreError,
    FindConfirmedBtcChatsForStoreOk,
}

impl fmt::Display for LogEvents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogEvents::FindConfirmedBtcChatsForStoreStart => write!(f, "FindConfirmedBtcChatsForStoreStart"),
            LogEvents::FindConfirmedBtcChatsForStoreError => write!(f, "FindConfirmedBtcChatsForStoreError"),
            LogEvents::FindConfirmedBtcChatsForStoreOk => write!(f, "FindConfirmedBtcChatsForStoreOk"),
        }
    }
}

