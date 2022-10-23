// Empty stub for now
use actix_web::{HttpRequest, HttpResponse};
use actix_web::http::StatusCode;
use actix_web::web::{Data, Payload};
use btcpay_rust::models::WebhookInvoiceReceivedPaymentEvent;
use std::fmt;
use actix::Addr;
use crate::{AppContext, MyReceivedBtcChatStreamer};
use crate::web::service::invoice::callback::find_btc_chat_for_callback::find_btc_chat_for_callback;
use crate::web::service::invoice::callback::send_update_to_user::send_update_to_user;
use crate::web::service::invoice::callback::update_btc_chat_processing_status::{update_btc_chat_processing_status};
use crate::web::service::invoice::callback::verify_and_deserialize_request_body::verify_and_deserialize_request_body;

pub async fn handle_invoice_received_payment_callback(req: HttpRequest,
                                                      mut body: Payload,
                                                      context: Data<AppContext>,
                                                      my_received_btc_chat_streamer: Data<Addr<MyReceivedBtcChatStreamer>>) -> HttpResponse {

    log::info!("{:?}",
               LogEvents::HandleInvoicePaymentCallbackStart);

    let result =
        do_handle_invoice_received_payment_callback(&req,
                                                    &mut body,
                                                    &context,
                                                    &my_received_btc_chat_streamer).await;

    match result {
        Ok(()) => {
            log::info!("{:?}",
                       LogEvents::HandleInvoicePaymentCallbackOk);
            HttpResponse::Ok().finish()
        },
        Err(error) => {
            log::error!("{:?} {:?} ",
                        LogEvents::HandleInvoicePaymentCallbackError,
                        error.1.clone());
            HttpResponse::build(error.0.clone())
                         .body(LogEvents::HandleInvoicePaymentCallbackError.to_string())
        }
    }
}


async fn do_handle_invoice_received_payment_callback(req: &HttpRequest,
                                                     body: &mut Payload,
                                                     context: &Data<AppContext>,
                                                     my_received_btc_chat_streamer: &Data<Addr<MyReceivedBtcChatStreamer>>)
        ->Result<(),
                (StatusCode, String)> {

    let webhook_invoice_processing_event :WebhookInvoiceReceivedPaymentEvent =
        verify_and_deserialize_request_body(req,
                                            body,
                                            context).await?;

    let mut btc_chat =
        find_btc_chat_for_callback(&webhook_invoice_processing_event,
                                           context)?;

    update_btc_chat_processing_status(&mut btc_chat,
                                      &context)?;

    let ok =
        send_update_to_user(&mut btc_chat,
                            &context,
                            &my_received_btc_chat_streamer).await;

    if ok.is_err() {
        log::warn!("{:?}, {:?}",
                   LogEvents::HandleInvoicePaymentCallbackUpdateUserFailure,
                   ok.err().unwrap().1)
    }

    Ok(())
}

#[derive(Debug)]
pub enum LogEvents {
    HandleInvoicePaymentCallbackStart,
    HandleInvoicePaymentCallbackError,
    HandleInvoicePaymentCallbackUpdateUserFailure,
    HandleInvoicePaymentCallbackOk,
}

impl fmt::Display for LogEvents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogEvents::HandleInvoicePaymentCallbackStart
                => write!(f, "HandleInvoicePaymentCallbackStart"),
            LogEvents::HandleInvoicePaymentCallbackError
                => write!(f, "HandleInvoicePaymentCallbackError"),
            LogEvents::HandleInvoicePaymentCallbackUpdateUserFailure
                => write!(f, "HandleInvoicePaymentCallbackUpdateUserFailure"),
            LogEvents::HandleInvoicePaymentCallbackOk
                => write!(f, "HandleInvoicePaymentCallbackOk"),
        }
    }
}

