use crate::data::model::btc_chat::{BtcChat, ProcessingStatus};
use crate::AppContext;
use std::fmt;
use actix_web::http::StatusCode;
use actix_web::web::Data;

pub fn update_btc_chat_processing_status(btc_chat: &mut BtcChat,
                                         context: &Data<AppContext>) -> Result<(),
                                                                 (StatusCode, String)> {

    log::info!("{:?}, {:?}",
               LogEvents::UpdateBtcChatProcessingStatusStart,
               btc_chat);

    if btc_chat.processing_status() != ProcessingStatus::UnConfirmed {
        log::info!("{:?}, {:?}",
                   LogEvents::UpdateBtcChatProcessingStatusIgnoringNonNewBtcChat,
                   btc_chat);
        return Ok(());
    }

    btc_chat.processing_status_code = ProcessingStatus::Confirmed.to_code();

    let result =
        context.repositories
            .btc_chat_repository
            .update(btc_chat);

    if result.is_err() {
        log::error!("{:?} {:?}",
                    LogEvents::UpdateBtcChatProcessingStatusError,
                    result.as_ref().err().unwrap().to_string());
        return Err((StatusCode::INTERNAL_SERVER_ERROR,
                    LogEvents::UpdateBtcChatProcessingStatusError.to_string()));
    }

    log::info!("{:?}",
               LogEvents::UpdateBtcChatProcessingStatusPaidOk);

    Ok(())
}



#[derive(Debug)]
enum LogEvents {
    UpdateBtcChatProcessingStatusStart,
    UpdateBtcChatProcessingStatusIgnoringNonNewBtcChat,
    UpdateBtcChatProcessingStatusError,
    UpdateBtcChatProcessingStatusPaidOk,
}

impl fmt::Display for LogEvents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogEvents::UpdateBtcChatProcessingStatusStart => write!(f, "UpdateBtcChatProcessingStatusStart"),
            LogEvents::UpdateBtcChatProcessingStatusIgnoringNonNewBtcChat => write!(f, "UpdateBtcChatProcessingStatusIgnoringNonNewBtcChat"),
            LogEvents::UpdateBtcChatProcessingStatusError => write!(f, "UpdateBtcChatProcessingStatusError"),
            LogEvents::UpdateBtcChatProcessingStatusPaidOk => write!(f, "UpdateBtcChatProcessingStatusPaidOk"),
        }
    }
}
