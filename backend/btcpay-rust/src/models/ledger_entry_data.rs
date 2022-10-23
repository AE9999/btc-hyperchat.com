/*
 * BTCPay Greenfield API
 *
 * A full API to use your BTCPay Server
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LedgerEntryData : A single ledger entry meaning an asset and qty has changed (increased or decreased).



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LedgerEntryData {
    /// An asset.
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,
    /// The quantity changed of the asset. Can be positive or negative.
    #[serde(rename = "qty", skip_serializing_if = "Option::is_none")]
    pub qty: Option<f32>,
    /// Trade, Fee or Withdrawal
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}

impl LedgerEntryData {
    /// A single ledger entry meaning an asset and qty has changed (increased or decreased).
    pub fn new() -> LedgerEntryData {
        LedgerEntryData {
            asset: None,
            qty: None,
            _type: None,
        }
    }
}


