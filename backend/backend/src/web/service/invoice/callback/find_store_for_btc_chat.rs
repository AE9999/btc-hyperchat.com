use actix_web::http::StatusCode;
use std::fmt;
use actix_web::web::Data;
use crate::AppContext;
use crate::data::model::btc_chat::BtcChat;
use crate::data::model::store::Store;

pub fn find_store_for_btc_chat(btc_chat: &BtcChat,
                               context: &Data<AppContext>) -> Result<Store,
                                                                    (StatusCode, String)> {
    log::info!("{:?} {:?}",
                LogEvents::FindStoreForeBtcChatStart,
                btc_chat);

    let store =
        context.repositories
            .store_repository
            .find_by_btcpay_store_id(btc_chat.store_id.as_str());

    if store.is_err() {
        log::error!("{:?} {:?}",
                    LogEvents::FindStoreForeBtcChatError,
                    store.as_ref().err().unwrap().to_string());
        return Err((StatusCode::INTERNAL_SERVER_ERROR,
                    LogEvents::FindStoreForeBtcChatError.to_string()))
    }

    let store = store.unwrap();

    if store.is_none() {
        log::error!("{:?}",
                    LogEvents::FindStoreForeBtcChatNoStore);
        return Err((StatusCode::BAD_REQUEST,
                    LogEvents::FindStoreForeBtcChatNoStore.to_string()));
    }

    let store = store.unwrap();

    log::info!("{:?} {:?}",
                LogEvents::FindStoreForeBtcChatOk,
                btc_chat);

    Ok(store)
}

#[derive(Debug)]
enum LogEvents {
    FindStoreForeBtcChatStart,
    FindStoreForeBtcChatError,
    FindStoreForeBtcChatNoStore,
    FindStoreForeBtcChatOk,
}

impl fmt::Display for LogEvents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogEvents::FindStoreForeBtcChatStart => write!(f, "FindStoreForeBtcChatStart"),
            LogEvents::FindStoreForeBtcChatError => write!(f, "FindStoreForeBtcChatError"),
            LogEvents::FindStoreForeBtcChatNoStore => write!(f, "FindStoreForeBtcChatNoStore"),
            LogEvents::FindStoreForeBtcChatOk => write!(f, "FindStoreForeBtcChatOk"),
        }
    }
}
