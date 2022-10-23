use actix_web::http::StatusCode;
use crate::AppContext;
use crate::data::model::btc_chat::{BtcChat,  ProcessingStatus};
use crate::web::model::btc_chat::register_btc_chat::register_btc_chat_request::RegisterBtcChatRequest;
use std::fmt;
use actix_web::web::{Data, Json};
use sanitize_html::sanitize_str;
use sanitize_html::rules::predefined::DEFAULT;

pub async fn create_btc_chat_in_db_from_invoice_and_register_btc_chat_request(invoice_id: &String,
                                                                              body: &Json<RegisterBtcChatRequest>,
                                                                              context: &Data<AppContext>)
                                                                              -> Result<BtcChat, (StatusCode, String)> {

    log::info!("{:?}, {:?}, {:?}",
                   LogEvents::CreateBtcChatInDbFromInvoiceAndRegisterBtcChatRequestStart,
                   invoice_id,
                   body);

    let message =
        sanatize_html(body.message.clone())?;

    let sender =
        sanatize_html(body.sender.clone())?;

    let btc_chat
        = context.repositories.btc_chat_repository.create(true,
                                                          ProcessingStatus::UnConfirmed.to_code(),
                                                          message,
                                                          sender,
                                                          body.store_id.clone(),
                                                          invoice_id.clone(),
                                                          0, // We don't converse sat for now
                                                        body.amount as i32,
                                                          body.currency.clone());

    if btc_chat.is_err() {
        log::error!("{:?}, {:?}",
                   LogEvents::CreateBtcChatInDbFromInvoiceAndRegisterBtcChatRequestError,
                   btc_chat.as_ref().err().unwrap().to_string());
        return Err((StatusCode::INTERNAL_SERVER_ERROR,
                    LogEvents::CreateBtcChatInDbFromInvoiceAndRegisterBtcChatRequestError.to_string()));
    }

    let btc_chat = btc_chat.unwrap();

    log::info!("{:?}",
               LogEvents::CreateBtcChatInDbFromInvoiceAndRegisterBtcChatRequestOk);

    Ok(btc_chat)
}

fn sanatize_html(input: Option<String>) -> Result<Option<String>,
                                                  (StatusCode, String)> {
    if input.is_none() {
        Ok(input)
    } else {
        let message = input.unwrap();
        let message = sanitize_str(&DEFAULT, message.as_str());
        if message.is_err() {
            log::error!("{:?} {:?}",
                        LogEvents::CreateBtcChatInDbFromInvoiceAndRegisterBtcChatRequestError.to_string(),
                        message.as_ref().err().unwrap());
            return Err((StatusCode::INTERNAL_SERVER_ERROR,
                        LogEvents::CreateBtcChatInDbFromInvoiceAndRegisterBtcChatRequestError.to_string()));
        }
        Ok(Some(message.unwrap()))
    }
}

#[derive(Debug)]
enum LogEvents {
    CreateBtcChatInDbFromInvoiceAndRegisterBtcChatRequestStart,
    CreateBtcChatInDbFromInvoiceAndRegisterBtcChatRequestError,
    CreateBtcChatInDbFromInvoiceAndRegisterBtcChatRequestOk,
}

impl fmt::Display for LogEvents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogEvents::CreateBtcChatInDbFromInvoiceAndRegisterBtcChatRequestStart
                => write!(f, "CreateBtcChatInDbFromInvoiceAndRegisterBtcChatRequestStart"),
            LogEvents::CreateBtcChatInDbFromInvoiceAndRegisterBtcChatRequestError
                => write!(f, "CreateBtcChatInDbFromInvoiceAndRegisterBtcChatRequestError"),
            LogEvents::CreateBtcChatInDbFromInvoiceAndRegisterBtcChatRequestOk
                => write!(f, "CreateBtcChatInDbFromInvoiceAndRegisterBtcChatRequestOk"),
        }
    }
}
