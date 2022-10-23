use std::fmt;
use actix_web::http::StatusCode;
use uuid::Uuid;
use std::str::FromStr;
use actix_web::web::Json;
use crate::web::model::btc_chat::process_btc_chat::process_btc_chat_request::ProcessBtcChatRequest;


pub fn extract_btc_chat_id_from_request(body: &Json<ProcessBtcChatRequest>) ->
                                                                            Result<Uuid,
                                                                                (StatusCode, String)>{

    log::info!("{:?} {:?}",
                LogEvents::ExtractUuidFromRequestStart,
                body);

    let uuid = Uuid::from_str(body.btc_chat_id.as_str());

    if uuid.is_err() {
        let message = uuid.err().unwrap().to_string();
        log::info!("{:?} {:?}",
                   LogEvents::ExtractUuidFromRequestError,
                   message);

        return Err((StatusCode::BAD_REQUEST,
                    LogEvents::ExtractUuidFromRequestError.to_string()))
    }

    log::info!("{:?}",
                LogEvents::ExtractUuidFromRequestOk);

    let uuid = uuid.unwrap();

    Ok(uuid)
}

#[derive(Debug)]
pub enum LogEvents {
    ExtractUuidFromRequestStart,
    ExtractUuidFromRequestOk,
    ExtractUuidFromRequestError,
}

impl fmt::Display for LogEvents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogEvents::ExtractUuidFromRequestStart => write!(f, "ExtractUuidFromRequestStart"),
            LogEvents::ExtractUuidFromRequestOk => write!(f, "ExtractUuidFromRequestOk"),
            LogEvents::ExtractUuidFromRequestError => write!(f, "ExtractUuidFromRequestError"),
        }
    }
}


