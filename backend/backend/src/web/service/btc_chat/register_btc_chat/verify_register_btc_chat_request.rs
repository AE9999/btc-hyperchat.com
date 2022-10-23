use actix_web::http::StatusCode;
use crate::AppContext;
use crate::web::model::btc_chat::register_btc_chat::register_btc_chat_request::RegisterBtcChatRequest;
use std::fmt;
use actix_web::web::{Data, Json};

pub fn verify_register_btc_chat_request(body: &Json<RegisterBtcChatRequest>,
                                        context: &Data<AppContext>) -> Result<(), (StatusCode, String)> {

    log::info!("{:?}, {:?}",
               LogEvents::VerifyRegisterBtcChatRequestStart,
               body);

    let _ok = verify_store_exists(&body.store_id, context)?;

    let _ok = verify_currency(&body.currency, context)?;

    let _ok = verify_amount(&body.amount, context)?;

    log::info!("{:?}",
               LogEvents::VerifyRegisterBtcChatRequestOk);

    return Ok(())
}


fn verify_store_exists(store_id: &String,
                       context: &Data<AppContext>) -> Result<(), (StatusCode, String)> {
    let store =
        context.repositories.store_repository.find_by_btcpay_store_id(store_id);

    if store.is_err() {
        log::error!("{:?}, {:?}",
                    LogEvents::VerifyRegisterBtcChatRequestNonExistingStoreDbFailure,
                    store.as_ref().err().unwrap().to_string());
        return Err((StatusCode::INTERNAL_SERVER_ERROR,
                    LogEvents::VerifyRegisterBtcChatRequestNonExistingStoreDbFailure.to_string()));
    }

    let store = store.unwrap();

    if store.is_none() {
        log::error!("{:?}, {:?}",
                   LogEvents::VerifyRegisterBtcChatRequestNonExistingStoreFailure,
                   store_id);
        return Err((StatusCode::BAD_REQUEST,
                    LogEvents::VerifyRegisterBtcChatRequestNonExistingStoreFailure.to_string()));
    }

    Ok(())
}

fn verify_currency(currency: &String, context: &Data<AppContext>) -> Result<(), (StatusCode, String)> {
    if !context.config.invoice_config.accepted_currencies.contains(currency) {
        log::info!("{:?} {:?}", LogEvents::VerifyRegisterBtcChatRequestWrongCurrencyFailure, currency);
        return Err((StatusCode::BAD_REQUEST,
                    LogEvents::VerifyRegisterBtcChatRequestWrongCurrencyFailure.to_string()));
    }
    Ok(())
}

fn verify_amount(amount: &u32, context: &Data<AppContext>) -> Result<(), (StatusCode, String)> {

    if amount < &context.config.invoice_config.min_amount {
        log::info!("{:?} {:?}", LogEvents::VerifyRegisterBtcChatRequestTooLowAmountFailure, amount);
        return Err((StatusCode::BAD_REQUEST,
                    LogEvents::VerifyRegisterBtcChatRequestTooLowAmountFailure.to_string()));
    }

    if amount > &context.config.invoice_config.max_amount {
        log::info!("{:?} {:?}", LogEvents::VerifyRegisterBtcChatRequestTooHighAmountFailure, amount);
        return Err((StatusCode::BAD_REQUEST,
                    LogEvents::VerifyRegisterBtcChatRequestTooHighAmountFailure.to_string()));
    }

    Ok(())
}

#[derive(Debug)]
enum LogEvents {
    VerifyRegisterBtcChatRequestStart,
    VerifyRegisterBtcChatRequestNonExistingStoreDbFailure,
    VerifyRegisterBtcChatRequestTooLowAmountFailure,
    VerifyRegisterBtcChatRequestTooHighAmountFailure,
    VerifyRegisterBtcChatRequestWrongCurrencyFailure,
    VerifyRegisterBtcChatRequestNonExistingStoreFailure,
    VerifyRegisterBtcChatRequestOk,
}

impl fmt::Display for LogEvents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogEvents::VerifyRegisterBtcChatRequestStart
                => write!(f, "VerifyRegisterBtcChatRequestStart"),
            LogEvents::VerifyRegisterBtcChatRequestNonExistingStoreDbFailure
            => write!(f, "VerifyRegisterBtcChatRequestNonExistingStoreDbFailure"),
            LogEvents::VerifyRegisterBtcChatRequestTooLowAmountFailure
                => write!(f, "VerifyRegisterBtcChatRequestTooLowAmountFailure"),
            LogEvents::VerifyRegisterBtcChatRequestTooHighAmountFailure
                => write!(f, "VerifyRegisterBtcChatRequestTooHighAmountFailure"),
            LogEvents::VerifyRegisterBtcChatRequestWrongCurrencyFailure
                => write!(f, "VerifyRegisterBtcChatRequestWrongCurrencyFailure"),
            LogEvents::VerifyRegisterBtcChatRequestNonExistingStoreFailure
                => write!(f, "VerifyRegisterBtcChatRequestNonExistingStoreFailure"),
            LogEvents::VerifyRegisterBtcChatRequestOk
                => write!(f, "VerifyRegisterBtcChatRequestOk"),
        }
    }
}
