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
pub struct LightningNetworkPaymentMethodBaseData {
    /// The lightning connection string. Set to 'Internal Node' to use the internal node. (See [this doc](https://github.com/btcpayserver/BTCPayServer.Lightning/blob/master/README.md#examples) for some example)
    #[serde(rename = "connectionString", skip_serializing_if = "Option::is_none")]
    pub connection_string: Option<String>,
    /// Whether to disable generation of bolt11 invoices. Useful when wanting to only use LNURL Pay exclusively.
    #[serde(rename = "disableBOLT11PaymentOption", skip_serializing_if = "Option::is_none")]
    pub disable_bolt11_payment_option: Option<bool>,
}

impl LightningNetworkPaymentMethodBaseData {
    pub fn new() -> LightningNetworkPaymentMethodBaseData {
        LightningNetworkPaymentMethodBaseData {
            connection_string: None,
            disable_bolt11_payment_option: None,
        }
    }
}


