use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MyReceivedBtcChat {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<String>,
    #[serde(rename = "amountOfSats")]
    pub amount_of_sats: i64,
    #[serde(rename = "amountInFiat")]
    pub amount_in_fiat: i32,
    pub currency: String,
    #[serde(rename = "dateCreated")]
    pub date_created: String,
}

impl MyReceivedBtcChat {
    pub fn new(id: String,
               message: Option<String>,
               sender:Option<String>,
               amount_of_sats: i64,
               amount_in_fiat: i32,
               currency: String,
               date_created: String) -> Self {
        MyReceivedBtcChat {
            id,
            message,
            sender,
            amount_of_sats,
            amount_in_fiat,
            currency,
            date_created
        }
    }
}
