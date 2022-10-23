use strum_macros::EnumIter;

#[derive(Debug, EnumIter)]
pub enum CallbackMapping {
    Created ,
    ReceivedPayment,
    Processing,
    Expired,
    Settled,
    Invalid,
}


impl CallbackMapping {
    pub fn path(&self) -> &str {
        match *self {
            CallbackMapping::Created => "/api/invoice/callback/created",
            CallbackMapping::ReceivedPayment => "/api/invoice/callback/receivedpayment",
            CallbackMapping::Processing => "/api/invoice/callback/processing",
            CallbackMapping::Expired => "/api/invoice/callback/expired",
            CallbackMapping::Settled => "/api/invoice/callback/settled",
            CallbackMapping::Invalid => "/api/invoice/callback/invalid",
        }
    }

    pub fn event_type(&self) -> &str {
        match *self {
            CallbackMapping::Created => "InvoiceCreated",
            CallbackMapping::ReceivedPayment => "InvoiceReceivedPayment",
            CallbackMapping::Processing => "InvoiceProcessing",
            CallbackMapping::Expired => "InvoiceExpired",
            CallbackMapping::Settled => "InvoiceSettled",
            CallbackMapping::Invalid => "InvoiceInvalid",
        }
    }

    pub fn is_received_payment(&self) -> bool {
        match *self {
            CallbackMapping::ReceivedPayment => true,
            _ => false,
        }
    }

    /*
    Fix this later
    pub fn event_method(&self) -> fn(HttpRequest, Data<Context>) -> Future<Output=> {
        match *self {
            CallbackMapping::Created => handle_invoice_created_callback,
            CallbackMapping::ReceivedPayment => handle_invoice_received_payment_callback,
            CallbackMapping::Processing => handle_invoice_processing_callback,
            CallbackMapping::Expired => handle_invoice_expired_callback,
            CallbackMapping::Settled => handle_invoice_settled_callback,
            CallbackMapping::Invalid => handle_invoice_invalid_callback,
        }
    }*/
}


