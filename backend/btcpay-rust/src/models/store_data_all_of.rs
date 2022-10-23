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
pub struct StoreDataAllOf {
    /// The id of the store
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl StoreDataAllOf {
    pub fn new() -> StoreDataAllOf {
        StoreDataAllOf {
            id: None,
        }
    }
}


