use std::collections::HashMap;
use crate::web::model::settings::store_webhook_config::post_type::{PostType, PostTypeValue};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct StoreWebhookConfig {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "dataAttributes", skip_serializing_if = "Option::is_none")]
    pub data_attributes: Option<HashMap<String, String>>,

    #[serde(rename = "queryAttributes", skip_serializing_if = "Option::is_none")]
    pub query_attributes: Option<HashMap<String, String>>,

    #[serde(rename = "headerAttributes", skip_serializing_if = "Option::is_none")]
    pub header_attributes: Option<HashMap<String, String>>,

    #[serde(rename = "postTypeCode", skip_serializing_if = "Option::is_none")]
    pub post_type_code: Option<PostTypeValue>
    
}

impl StoreWebhookConfig {

    pub fn new(url: Option<String>,
               data_attributes: Option<HashMap<String, String>>,
               query_attributes: Option<HashMap<String, String>>,
               header_attributes : Option<HashMap<String, String>>,
               post_type: Option<PostType>) -> Self {
        StoreWebhookConfig {
            url,
            data_attributes,
            query_attributes,
            header_attributes,
            post_type_code: if post_type.is_some() {
                                Some(post_type.as_ref().unwrap().to_code())
                            } else{
                                None
                            },
        }
    }

    pub fn is_strictly_empty(&self) -> bool {
        self.url.is_none()
        && self.data_attributes.is_none()
        && self.query_attributes.is_none()
        && self.header_attributes.is_none()
        && self.post_type_code.is_none()
    }

    pub fn has_minimally_needed_attributes(&self) -> bool {
        self.url.is_some()
            && self.post_type_code.is_some()
            && !PostType::from_code(*(self.post_type_code.as_ref().unwrap())).is_undefined()
    }

    pub fn is_correct(&self) -> bool {
        self.is_strictly_empty()
        || self.has_minimally_needed_attributes()
    }
}
