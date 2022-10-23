use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProcessBtcChatRequest {
    #[serde(rename = "btcChatId")]
    pub btc_chat_id: String,
}
