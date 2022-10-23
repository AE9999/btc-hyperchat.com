use actix_web::HttpResponse;
use actix_web::http::StatusCode;
use actix_web::web::{Data, Json};
use crate::AppContext;
use crate::web::model::btc_chat::process_btc_chat::process_btc_chat_request::ProcessBtcChatRequest;
use actix_web_middleware_keycloak_auth::StandardKeycloakClaims;
use crate::data::model::btc_chat::BtcChat;
use crate::web::model::btc_chat::process_btc_chat::process_btc_chat_response::ProcessBtcChatResponse;
use crate::web::service::btc_chat::process_btc_chat::verify_poster_is_owner_of_chat::verify_poster_is_owner_of_chat;
use crate::web::service::btc_chat::process_btc_chat::extract_btc_chat_id_from_request::extract_btc_chat_id_from_request;
use crate::web::service::btc_chat::process_btc_chat::find_btc_chat_by_id::find_btc_chat_by_id;
use crate::web::service::btc_chat::process_btc_chat::set_chat_status_as_processed_in_db::set_chat_status_as_processed_in_db;
use std::fmt;

pub async fn process_btc_chat(body: Json<ProcessBtcChatRequest>,
                              claims: StandardKeycloakClaims,
                              context: Data<AppContext>) -> HttpResponse {

    log::info!("{:?}",
               LogEvents::ProcessBtcChatStart);

    let result = do_process_btc_chat(&body,
                                                         &claims,
                                                         &context).await;

    match result {
        Ok(process_btc_chat_response) => {
            log::info!("{:?} {:?} ",
                       LogEvents::ProcessBtcChatOk,
                       process_btc_chat_response);
            HttpResponse::Ok().json(process_btc_chat_response)
        },
        Err(error) => {
            log::error!("{:?} {:?} ",
                        LogEvents::ProcessBtcChatError,
                        error.1.clone());
            HttpResponse::build(error.0.clone())
                .body(LogEvents::ProcessBtcChatError.to_string())
        }
    }
}



async fn do_process_btc_chat(body: &Json<ProcessBtcChatRequest>,
                             claims: &StandardKeycloakClaims,
                             context: &Data<AppContext>) -> Result<ProcessBtcChatResponse,
                                                                (StatusCode, String)> {
    let btc_chat_id = extract_btc_chat_id_from_request(&body)?;

    let mut btc_chat : BtcChat = find_btc_chat_by_id(&btc_chat_id,
                                                 context)?;

    verify_poster_is_owner_of_chat(&claims.sub,
                                           &btc_chat,
                                           context)?;

    set_chat_status_as_processed_in_db(&mut btc_chat,
                                 context)?;

    let process_btc_chat_response =
        ProcessBtcChatResponse::new(btc_chat.id.to_string());

    return Ok(process_btc_chat_response);
}

#[derive(Debug)]
pub enum LogEvents {
    ProcessBtcChatStart,
    ProcessBtcChatOk,
    ProcessBtcChatError,
}

impl fmt::Display for LogEvents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogEvents::ProcessBtcChatStart => write!(f, "ProcessBtcChatStart"),
            LogEvents::ProcessBtcChatOk => write!(f, "ProcessBtcChatOk"),
            LogEvents::ProcessBtcChatError => write!(f, "ProcessBtcChatError"),

        }
    }
}

