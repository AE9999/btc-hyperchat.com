use actix_web::http::StatusCode;
use uuid::Uuid;
use crate::data::model::btc_chat::BtcChat;
use crate::AppContext;
use std::fmt;
use actix_web::web::Data;


pub fn find_btc_chat_by_id(btc_chat_id: &Uuid,
                           context: &Data<AppContext>) -> Result<BtcChat, (StatusCode, String)> {
    log::info!("{:?} {:?}",
                LogEvents::FindBtcChatByIdStart,
                btc_chat_id);

    let btc_chat =
        context.repositories.btc_chat_repository
               .find_by_pk(btc_chat_id);

    if btc_chat.is_err() {

        log::error!("{:?} {:?}",
                LogEvents::FindBtcChatByIdDbError,
                btc_chat.as_ref().err().unwrap().to_string());

        return Err((StatusCode::INTERNAL_SERVER_ERROR,
                    LogEvents::FindBtcChatByIdDbError.to_string()))
    }

    let btc_chat = btc_chat.unwrap();

    if btc_chat.is_none() {
        log::info!("{:?}",
                LogEvents::FindBtcChatByIdNoChatFound);

        return Err((StatusCode::BAD_REQUEST,
                    LogEvents::FindBtcChatByIdNoChatFound.to_string()))
    }

    let btc_chat = btc_chat.unwrap();

    log::info!("{:?} {:?}",
                LogEvents::FindBtcChatByIdOk,
                btc_chat);


    Ok(btc_chat)
}


#[derive(Debug)]
enum LogEvents {
    FindBtcChatByIdStart,
    FindBtcChatByIdDbError,
    FindBtcChatByIdNoChatFound,
    FindBtcChatByIdOk,
}

impl fmt::Display for LogEvents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogEvents::FindBtcChatByIdStart => write!(f, "FindBtcChatByIdStart"),
            LogEvents::FindBtcChatByIdDbError => write!(f, "FindBtcChatByIdDbError"),
            LogEvents::FindBtcChatByIdNoChatFound => write!(f, "FindBtcChatByIdNoChatFound"),
            LogEvents::FindBtcChatByIdOk => write!(f, "FindBtcChatByIdOk"),
        }
    }
}
