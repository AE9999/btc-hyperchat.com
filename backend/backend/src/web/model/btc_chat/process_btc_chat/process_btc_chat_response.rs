use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProcessBtcChatResponse {
    #[serde(rename = "btcChatId")]
    pub btc_chat_id: String,
}

impl ProcessBtcChatResponse {
    pub fn new(btc_chat_id: String) -> Self {
        ProcessBtcChatResponse {
            btc_chat_id
        }
    }
}
