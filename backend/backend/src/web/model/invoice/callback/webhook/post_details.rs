use std::collections::HashMap;
use crate::web::model::settings::store_webhook_config::post_type::PostType;

#[derive(Debug, Clone)]
pub struct PostDetails {
    pub url: String,
    pub post_type: PostType,
    pub header_elements: HashMap<String, String>,
    pub body: HashMap<String, String>,
    pub query_elements: HashMap<String, String>,
}

impl PostDetails {
    pub fn new(url:String,
               post_type:PostType,
               header_elements: HashMap<String, String>,
               body: HashMap<String, String>,
               query_elements: HashMap<String, String>,) -> Self {
        PostDetails {
            url,
            post_type,
            header_elements,
            body,
            query_elements,
        }
    }
}
