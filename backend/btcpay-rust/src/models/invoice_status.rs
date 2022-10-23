/*
 * BTCPay Greenfield API
 *
 * A full API to use your BTCPay Server
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// InvoiceStatus : 

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InvoiceStatus {
    #[serde(rename = "New")]
    New,
    #[serde(rename = "Processing")]
    Processing,
    #[serde(rename = "Expired")]
    Expired,
    #[serde(rename = "Invalid")]
    Invalid,
    #[serde(rename = "Settled")]
    Settled,

}

impl ToString for InvoiceStatus {
    fn to_string(&self) -> String {
        match self {
            Self::New => String::from("New"),
            Self::Processing => String::from("Processing"),
            Self::Expired => String::from("Expired"),
            Self::Invalid => String::from("Invalid"),
            Self::Settled => String::from("Settled"),
        }
    }
}

impl Default for InvoiceStatus {
    fn default() -> InvoiceStatus {
        Self::New
    }
}




