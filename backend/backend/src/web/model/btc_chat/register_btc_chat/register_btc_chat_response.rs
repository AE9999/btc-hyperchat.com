use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SubmitBtcChatResponse {
    #[serde(rename = "invoiceId")]
    invoice_id: String,
}

impl SubmitBtcChatResponse {
    pub fn new(invoice_id: String) -> Self {
        SubmitBtcChatResponse {
            invoice_id
        }
    }
}
