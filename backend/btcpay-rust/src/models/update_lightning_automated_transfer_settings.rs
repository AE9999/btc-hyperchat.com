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
pub struct UpdateLightningAutomatedTransferSettings {
    /// How often should the processor run
    #[serde(rename = "intervalSeconds", skip_serializing_if = "Option::is_none")]
    pub interval_seconds: Option<Box<TimeSpanSeconds>>,
}

impl UpdateLightningAutomatedTransferSettings {
    pub fn new() -> UpdateLightningAutomatedTransferSettings {
        UpdateLightningAutomatedTransferSettings {
            interval_seconds: None,
        }
    }
}

