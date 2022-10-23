/*
 * BTCPay Greenfield API
 *
 * A full API to use your BTCPay Server
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OnchainBalanceData {
    /// The confirmed amount in satoshi
    #[serde(rename = "confirmed", skip_serializing_if = "Option::is_none")]
    pub confirmed: Option<String>,
    /// The unconfirmed amount in satoshi
    #[serde(rename = "unconfirmed", skip_serializing_if = "Option::is_none")]
    pub unconfirmed: Option<String>,
    /// The reserved amount in satoshi
    #[serde(rename = "reserved", skip_serializing_if = "Option::is_none")]
    pub reserved: Option<String>,
}

impl OnchainBalanceData {
    pub fn new() -> OnchainBalanceData {
        OnchainBalanceData {
            confirmed: None,
            unconfirmed: None,
            reserved: None,
        }
    }
}

