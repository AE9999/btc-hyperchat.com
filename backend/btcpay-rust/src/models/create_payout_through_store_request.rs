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
pub struct CreatePayoutThroughStoreRequest {
    /// The destination of the payout (can be an address or a BIP21 url)
    #[serde(rename = "destination", skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    /// The amount of the payout in the currency of the pull payment (eg. USD).
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    /// The payment method of the payout
    #[serde(rename = "paymentMethod", skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,
    /// The pull payment to create this for. Optional.
    #[serde(rename = "pullPaymentId", skip_serializing_if = "Option::is_none")]
    pub pull_payment_id: Option<String>,
    /// Whether to approve this payout automatically upon creation
    #[serde(rename = "approved", skip_serializing_if = "Option::is_none")]
    pub approved: Option<bool>,
}

impl CreatePayoutThroughStoreRequest {
    pub fn new() -> CreatePayoutThroughStoreRequest {
        CreatePayoutThroughStoreRequest {
            destination: None,
            amount: None,
            payment_method: None,
            pull_payment_id: None,
            approved: None,
        }
    }
}


