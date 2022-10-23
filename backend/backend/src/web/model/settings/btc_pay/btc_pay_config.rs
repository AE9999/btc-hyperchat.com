#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct BtcPayConfig {
    username: String,

    #[serde(rename = "initialPassword")]
    initial_password: String,

    #[serde(rename = "serverUrl")]
    server_url: String
}

impl BtcPayConfig {
    pub fn new(username: String,
               initial_password : String,
               server_url: String) -> Self {
        BtcPayConfig {
            username,
            initial_password,
            server_url
        }
    }
}
