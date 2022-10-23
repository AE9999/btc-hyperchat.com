#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct FindProfileNamesResponse {
    #[serde(rename = "profileNames")]
    profile_names: Vec<String>
}

impl FindProfileNamesResponse {
    pub fn new(profiles: Vec<String>) ->Self {
        FindProfileNamesResponse {
            profile_names: profiles
        }
    }
}
