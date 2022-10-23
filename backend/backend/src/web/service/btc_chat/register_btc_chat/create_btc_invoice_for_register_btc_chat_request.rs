use crate::web::model::btc_chat::register_btc_chat::register_btc_chat_request::RegisterBtcChatRequest;
use crate::AppContext;
use actix_web::http::StatusCode;
use btcpay_rust::apis::invoices_api::invoices_create_invoice;
use btcpay_rust::models::{CreateInvoiceRequest, InvoiceData};
use std::fmt;
use actix_web::web::{Data, Json};

pub async fn create_btc_invoice_for_register_btc_chat_request(body: &Json<RegisterBtcChatRequest>,
                                                              context: &Data<AppContext>) -> Result<InvoiceData,
                                                                                              (StatusCode, String)> {
    log::info!("{:?}, {:?}",
               LogEvents::CreateBtcInvoiceForRegisterBtcChatRequestStart,
               body);

    let create_invoice_request =
        create_invoice_request_from_submit_btc_chat_request(body);

    let invoice
        = invoices_create_invoice(&context.clients.btc_pay_configuration,
                                  &body.store_id,
                                  create_invoice_request).await;

    if invoice.is_err() {
        let message = invoice.as_ref().err().unwrap().to_string();
        log::info!("{:?} {:?}", LogEvents::CreateBtcInvoiceForRegisterBtcChatRequestError, message);
        return Err((StatusCode::BAD_REQUEST,
                    LogEvents::CreateBtcInvoiceForRegisterBtcChatRequestError.to_string()));
    }

    let invoice = invoice.unwrap();

    log::info!("{:?}",
               LogEvents::CreateBtcInvoiceForRegisterBtcChatRequestOk);

    Ok(invoice)
}

fn create_invoice_request_from_submit_btc_chat_request(body: &Json<RegisterBtcChatRequest>)
                                                       -> CreateInvoiceRequest  {
    let mut create_invoice_request =     CreateInvoiceRequest::new();
    create_invoice_request.amount = Some(body.amount.to_string());
    create_invoice_request.currency = Some(body.currency.clone());

    create_invoice_request
}

#[derive(Debug)]
enum LogEvents {
    CreateBtcInvoiceForRegisterBtcChatRequestStart,
    CreateBtcInvoiceForRegisterBtcChatRequestError,
    CreateBtcInvoiceForRegisterBtcChatRequestOk,
}

impl fmt::Display for LogEvents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogEvents::CreateBtcInvoiceForRegisterBtcChatRequestStart
            => write!(f, "CreateBtcInvoiceForRegisterBtcChatRequestStart"),
            LogEvents::CreateBtcInvoiceForRegisterBtcChatRequestError
            => write!(f, "CreateBtcInvoiceForRegisterBtcChatRequestError"),
            LogEvents::CreateBtcInvoiceForRegisterBtcChatRequestOk
            => write!(f, "CreateBtcInvoiceForRegisterBtcChatRequestOk"),
        }
    }
}

