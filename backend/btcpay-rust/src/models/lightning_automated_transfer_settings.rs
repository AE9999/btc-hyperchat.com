/*
 * BTCPay Greenfield API
 *
 * A full API to use your BTCPay Server
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */
use crate::models::time_span_seconds::TimeSpanSeconds;



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LightningAutomatedTransferSettings {
    /// payment method of the payout processor
    #[serde(rename = "paymentMethod", skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,
    /// How often should the processor run
    #[serde(rename = "intervalSeconds", skip_serializing_if = "Option::is_none")]
    pub interval_seconds: Option<Box<TimeSpanSeconds>>,
}

impl LightningAutomatedTransferSettings {
    pub fn new() -> LightningAutomatedTransferSettings {
        LightningAutomatedTransferSettings {
            payment_method: None,
            interval_seconds: None,
        }
    }
}


