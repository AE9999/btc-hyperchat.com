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
pub struct UpdateOnChainPaymentMethodRequest {
    /// Whether the payment method is enabled
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// The derivation scheme
    #[serde(rename = "derivationScheme", skip_serializing_if = "Option::is_none")]
    pub derivation_scheme: Option<String>,
    /// A label that will be shown in the UI
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// The wallet fingerprint followed by the keypath to derive the account key used for signing operation or creating PSBTs
    #[serde(rename = "accountKeyPath", skip_serializing_if = "Option::is_none")]
    pub account_key_path: Option<String>,
}

impl UpdateOnChainPaymentMethodRequest {
    pub fn new() -> UpdateOnChainPaymentMethodRequest {
        UpdateOnChainPaymentMethodRequest {
            enabled: None,
            derivation_scheme: None,
            label: None,
            account_key_path: None,
        }
    }
}


