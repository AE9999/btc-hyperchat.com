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
pub struct GenericPaymentMethodData {
    /// Whether the payment method is enabled
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// The currency code of the payment method
    #[serde(rename = "cryptoCode", skip_serializing_if = "Option::is_none")]
    pub crypto_code: Option<String>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<crate::models::GenericPaymentMethodDataData>>,
}

impl GenericPaymentMethodData {
    pub fn new() -> GenericPaymentMethodData {
        GenericPaymentMethodData {
            enabled: None,
            crypto_code: None,
            data: None,
        }
    }
}

