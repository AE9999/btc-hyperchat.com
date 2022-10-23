use btcpay_rust::models::{WebhookInvoiceEvent, WebhookInvoiceExpiredEvent, WebhookInvoiceInvalidEvent, WebhookInvoiceProcessingEvent, WebhookInvoiceReceivedPaymentEvent, WebhookInvoiceSettledEvent};
use crate::data::model::btc_chat::BtcChat;
use crate::AppContext;
use actix_web::http::StatusCode;
use std::fmt;
use actix_web::web::Data;

pub fn find_btc_chat_for_callback(webhook: &dyn ExtractableInvoiceId,
                                  context: &Data<AppContext>) -> Result<BtcChat,
                                                                    (StatusCode, String)> {
    log::info!("{:?}", LogEvents::FindInvoiceForCallbackStart);
    let invoice_id  = webhook.invoice_id();

    let btc_chat =
        context.repositories.btc_chat_repository
               .find_by_invoice_id(invoice_id.as_str());

    if btc_chat.is_err() {

        log::error!("{:?} {:?} ",
                    LogEvents::FindInvoiceForCallbackError,
                    btc_chat.err().unwrap().to_string());
        return Err((StatusCode::INTERNAL_SERVER_ERROR,
                    LogEvents::FindInvoiceForCallbackError.to_string()));
    }

    let btc_chat = btc_chat.unwrap();

    if btc_chat.is_none() {
        log::error!("{:?}",
                    LogEvents::FindInvoiceForCallbackNotFound);
        return Err((StatusCode::INTERNAL_SERVER_ERROR,
                    LogEvents::FindInvoiceForCallbackNotFound.to_string()));
    }

    let btc_chat = btc_chat.unwrap();

    log::info!("{:?}, {:?}", LogEvents::FindInvoiceForCallbackOk, btc_chat);

    Ok(btc_chat)

}

#[derive(Debug)]
enum LogEvents {
    FindInvoiceForCallbackStart,
    FindInvoiceForCallbackNotFound,
    FindInvoiceForCallbackError,
    FindInvoiceForCallbackOk,
}

impl fmt::Display for LogEvents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogEvents::FindInvoiceForCallbackStart => write!(f, "FindInvoiceForCallbackStart"),
            LogEvents::FindInvoiceForCallbackNotFound => write!(f, "FindInvoiceForCallbackNotFound"),
            LogEvents::FindInvoiceForCallbackError => write!(f, "FindInvoiceForCallbackError"),
            LogEvents::FindInvoiceForCallbackOk => write!(f, "FindInvoiceForCallbackOk"),
        }
    }
}


pub trait ExtractableInvoiceId {
    fn invoice_id(&self)  -> String;
}

//Inspired by https://stackoverflow.com/questions/39150216/implementing-a-trait-for-multiple-types-at-once

macro_rules! impl_ExtractableInvoiceId {
    (for $($t:ty),+) => {
        $(impl ExtractableInvoiceId for $t {
            fn invoice_id(&self) -> String {
                self.invoice_id.as_ref().expect("Callbacks have an invoice id").clone()
            }
        })*
    }
}

impl_ExtractableInvoiceId!(for WebhookInvoiceEvent,
                               WebhookInvoiceExpiredEvent,
                               WebhookInvoiceInvalidEvent,
                               WebhookInvoiceProcessingEvent,
                               WebhookInvoiceReceivedPaymentEvent,
                               WebhookInvoiceSettledEvent);
