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
pub struct LightningChannelData {
    /// The public key of the node (Node ID)
    #[serde(rename = "remoteNode", skip_serializing_if = "Option::is_none")]
    pub remote_node: Option<String>,
    /// Whether the node is public
    #[serde(rename = "isPublic", skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
    /// Whether the node is online
    #[serde(rename = "isActive", skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    /// The capacity of the channel in millisatoshi
    #[serde(rename = "capacity", skip_serializing_if = "Option::is_none")]
    pub capacity: Option<String>,
    /// The local balance of the channel in millisatoshi
    #[serde(rename = "localBalance", skip_serializing_if = "Option::is_none")]
    pub local_balance: Option<String>,
    #[serde(rename = "channelPoint", skip_serializing_if = "Option::is_none")]
    pub channel_point: Option<String>,
}

impl LightningChannelData {
    pub fn new() -> LightningChannelData {
        LightningChannelData {
            remote_node: None,
            is_public: None,
            is_active: None,
            capacity: None,
            local_balance: None,
            channel_point: None,
        }
    }
}


