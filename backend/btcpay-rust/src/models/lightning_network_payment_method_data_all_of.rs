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
pub struct LightningNetworkPaymentMethodDataAllOf {
    /// Whether the payment method is enabled
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Crypto code of the payment method
    #[serde(rename = "cryptoCode", skip_serializing_if = "Option::is_none")]
    pub crypto_code: Option<String>,
    /// The payment method
    #[serde(rename = "paymentMethod", skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,
}

impl LightningNetworkPaymentMethodDataAllOf {
    pub fn new() -> LightningNetworkPaymentMethodDataAllOf {
        LightningNetworkPaymentMethodDataAllOf {
            enabled: None,
            crypto_code: None,
            payment_method: None,
        }
    }
}


