use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct StoreWebhookActivation {
    #[serde(rename = "webhookActive")]
    pub webhook_active: bool,

    #[serde(rename = "automaticallyProcessBtcChatsIfWebhookSucceeds")]
    pub automatically_process_btc_chats_if_webhook_succeeds: bool,
}

impl StoreWebhookActivation{
    #[allow(dead_code)]
    pub fn new (webhook_active: bool,
                automatically_process_btc_chats_if_webhook_succeeds: bool) -> Self {
        StoreWebhookActivation {
            webhook_active,
            automatically_process_btc_chats_if_webhook_succeeds,
        }
    }
}
