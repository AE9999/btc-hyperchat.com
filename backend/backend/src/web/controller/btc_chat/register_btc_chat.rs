use std::fmt;
use actix_web::HttpResponse;
use actix_web::http::StatusCode;
use actix_web::web::{Data, Json};
use crate::AppContext;
use crate::web::model::btc_chat::register_btc_chat::register_btc_chat_request::RegisterBtcChatRequest;
use crate::web::model::btc_chat::register_btc_chat::register_btc_chat_response::SubmitBtcChatResponse;
use crate::web::service::btc_chat::register_btc_chat::create_btc_chat_in_db_from_invoice_and_register_btc_chat_request::create_btc_chat_in_db_from_invoice_and_register_btc_chat_request;
use crate::web::service::btc_chat::register_btc_chat::create_btc_invoice_for_register_btc_chat_request::create_btc_invoice_for_register_btc_chat_request;
use crate::web::service::btc_chat::register_btc_chat::verify_register_btc_chat_request::verify_register_btc_chat_request;

pub async fn register_btc_chat(body: Json<RegisterBtcChatRequest>, context: Data<AppContext>) -> HttpResponse {

    log::info!("{:?} {:?} ",
               LogEvents::RegisterBtcChatStart,
               body.0);

    let result =
        do_submit_btc_chat(&body,
                           &context).await;

    match result {
        Ok(register_tc_chat_response) => {
            log::info!("{:?} {:?} ",
                       LogEvents::RegisterBtcChatOk,
                       register_tc_chat_response);
            HttpResponse::Ok().json(register_tc_chat_response)
        },
        Err(error) => {
            log::error!("{:?} {:?} ",
                        LogEvents::RegisterBtcChatError,
                        error.1.clone());
            HttpResponse::build(error.0.clone())
                         .body(LogEvents::RegisterBtcChatError.to_string())
        }
    }
}

async fn do_submit_btc_chat(body: &Json<RegisterBtcChatRequest>,
                            context: &Data<AppContext>) -> Result<SubmitBtcChatResponse,
                                                               (StatusCode, String)> {


    verify_register_btc_chat_request(body, context)?;

    let invoice =
        create_btc_invoice_for_register_btc_chat_request(body,
                                                         context).await?;

    let invoice_id = invoice.id.unwrap();

    create_btc_chat_in_db_from_invoice_and_register_btc_chat_request(&invoice_id,
                                                                     body,
                                                                     context).await?;

    let register_btc_chat_response =
        SubmitBtcChatResponse::new(invoice_id.clone());

    Ok(register_btc_chat_response)
}

#[derive(Debug)]
pub enum LogEvents {
    RegisterBtcChatStart,
    RegisterBtcChatError,
    RegisterBtcChatOk,
}

impl fmt::Display for LogEvents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogEvents::RegisterBtcChatStart => write!(f, "RegisterBtcChatStart"),
            LogEvents::RegisterBtcChatError => write!(f, "RegisterBtcChatError"),
            LogEvents::RegisterBtcChatOk => write!(f, "RegisterBtcChatOk"),

        }
    }
}

