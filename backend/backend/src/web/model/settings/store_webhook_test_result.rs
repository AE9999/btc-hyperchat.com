use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct StoreWebhookTestResult {
    success: bool
}

impl StoreWebhookTestResult {
    pub fn new(success: bool) -> Self {
        StoreWebhookTestResult {
            success
        }
    }
}
