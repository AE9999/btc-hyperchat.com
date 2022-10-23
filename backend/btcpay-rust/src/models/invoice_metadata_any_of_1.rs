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
pub struct InvoiceMetadataAnyOf1 {
    #[serde(rename = "posData", skip_serializing_if = "Option::is_none")]
    pub pos_data: Option<String>, // Could not be rendered
}

impl InvoiceMetadataAnyOf1 {
    pub fn new() -> InvoiceMetadataAnyOf1 {
        InvoiceMetadataAnyOf1 {
            pos_data: None,
        }
    }
}


