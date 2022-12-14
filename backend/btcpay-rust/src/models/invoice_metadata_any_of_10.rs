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
pub struct InvoiceMetadataAnyOf10 {
    #[serde(rename = "buyerPhone", skip_serializing_if = "Option::is_none")]
    pub buyer_phone: Option<String>,
}

impl InvoiceMetadataAnyOf10 {
    pub fn new() -> InvoiceMetadataAnyOf10 {
        InvoiceMetadataAnyOf10 {
            buyer_phone: None,
        }
    }
}


