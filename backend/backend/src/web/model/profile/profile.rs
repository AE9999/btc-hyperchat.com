#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Profile {
    pub username: String,
    #[serde(rename = "storeId")]
    pub store_id: String,
    pub currencies: Vec<String>,
}

impl Profile {
    pub fn new(username: String,
               store_id:String,
                currencies: Vec<String>) -> Self {
        Profile {
            username,
            store_id,
            currencies,
        }
    }
}
