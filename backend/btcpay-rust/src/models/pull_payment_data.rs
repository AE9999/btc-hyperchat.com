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
pub struct PullPaymentData {
    /// Id of the pull payment
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Name given to pull payment when it was created
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Description given to pull payment when it was created
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The currency of the pull payment's amount
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// The amount in the currency of this pull payment as a decimal string
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    /// The length of each period in seconds
    #[serde(rename = "period", skip_serializing_if = "Option::is_none")]
    pub period: Option<i32>,
    /// If lightning is activated, do not accept BOLT11 invoices with expiration less than … days
    #[serde(rename = "BOLT11Expiration", skip_serializing_if = "Option::is_none")]
    pub bolt11_expiration: Option<String>,
    /// Any payouts created for this pull payment will skip the approval phase upon creation
    #[serde(rename = "autoApproveClaims", skip_serializing_if = "Option::is_none")]
    pub auto_approve_claims: Option<bool>,
    /// Whether this pull payment is archived
    #[serde(rename = "archived", skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    /// The link to a page to claim payouts to this pull payment
    #[serde(rename = "viewLink", skip_serializing_if = "Option::is_none")]
    pub view_link: Option<String>,
}

impl PullPaymentData {
    pub fn new() -> PullPaymentData {
        PullPaymentData {
            id: None,
            name: None,
            description: None,
            currency: None,
            amount: None,
            period: None,
            bolt11_expiration: None,
            auto_approve_claims: None,
            archived: None,
            view_link: None,
        }
    }
}


