use actix_web::http::StatusCode;
use crate::data::model::btc_chat::{BtcChat, ProcessingStatus};
use crate::AppContext;
use std::fmt;
use actix_web::web::Data;

pub fn set_chat_status_as_processed_in_db(btc_chat:&mut BtcChat,
                                          context: &Data<AppContext>) ->
                                      Result<(),
                                          (StatusCode, String)> {

    log::info!("{:?} {:?}",
                LogEvents::SetChatStatusAsProcessedInDbStart,
                btc_chat);

    btc_chat.processing_status_code = ProcessingStatus::ProcessedByCreator.to_code();

    let ok = context.repositories.btc_chat_repository.update(btc_chat);

    if ok.is_err() {

        log::error!("{:?} {:?}",
                    LogEvents::SetChatStatusAsProcessedInDbError,
                    ok.as_ref().err().unwrap().to_string());

        return Err((StatusCode::INTERNAL_SERVER_ERROR,
                    LogEvents::SetChatStatusAsProcessedInDbError.to_string()))
    }

    log::info!("{:?}",
                LogEvents::SetChatStatusAsProcessedInDbOk);

    Ok(())
}


#[derive(Debug)]
enum LogEvents {
    SetChatStatusAsProcessedInDbStart,
    SetChatStatusAsProcessedInDbError,
    SetChatStatusAsProcessedInDbOk,
}

impl fmt::Display for LogEvents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogEvents::SetChatStatusAsProcessedInDbStart => write!(f, "SetChatStatusAsProcessedInDbStart"),
            LogEvents::SetChatStatusAsProcessedInDbError => write!(f, "SetChatStatusAsProcessedInDbError"),
            LogEvents::SetChatStatusAsProcessedInDbOk => write!(f, "SetChatStatusAsProcessedInDbOk"),
        }
    }
}

