use serde::{Serialize, Deserialize};
use validator::{Validate};

#[derive(Serialize, Validate, Deserialize, Debug)]
pub struct RegisterBtcChatRequest {

    #[serde(rename = "storeId")]
    pub store_id: String,

    pub currency: String,

    pub amount: u32,

    #[validate(length(max = 200))]
    pub message: Option<String>,

    #[validate(length(min =2, max = 25))]
    pub sender: Option<String>,
}


