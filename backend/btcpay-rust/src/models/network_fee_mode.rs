/*
 * BTCPay Greenfield API
 *
 * A full API to use your BTCPay Server
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// NetworkFeeMode : Check whether network fee should be added to the invoice if on-chain payment is used. ([More information](https://docs.btcpayserver.org/FAQ/Stores/#add-network-fee-to-invoice-vary-with-mining-fees))

/// Check whether network fee should be added to the invoice if on-chain payment is used. ([More information](https://docs.btcpayserver.org/FAQ/Stores/#add-network-fee-to-invoice-vary-with-mining-fees))
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NetworkFeeMode {
    #[serde(rename = "MultiplePaymentsOnly")]
    MultiplePaymentsOnly,
    #[serde(rename = "Always")]
    Always,
    #[serde(rename = "Never")]
    Never,

}

impl ToString for NetworkFeeMode {
    fn to_string(&self) -> String {
        match self {
            Self::MultiplePaymentsOnly => String::from("MultiplePaymentsOnly"),
            Self::Always => String::from("Always"),
            Self::Never => String::from("Never"),
        }
    }
}

impl Default for NetworkFeeMode {
    fn default() -> NetworkFeeMode {
        Self::MultiplePaymentsOnly
    }
}




