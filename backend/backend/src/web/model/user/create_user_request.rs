use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserRequest {
    pub username: String,

    pub email: String,

    #[serde(rename = "keycloakId")]
    pub keycloak_id: String,
}
