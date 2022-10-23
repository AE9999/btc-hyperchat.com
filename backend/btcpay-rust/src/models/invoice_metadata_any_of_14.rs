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
pub struct InvoiceMetadataAnyOf14 {
    #[serde(rename = "taxIncluded", skip_serializing_if = "Option::is_none")]
    pub tax_included: Option<f32>,
}

impl InvoiceMetadataAnyOf14 {
    pub fn new() -> InvoiceMetadataAnyOf14 {
        InvoiceMetadataAnyOf14 {
            tax_included: None,
        }
    }
}

