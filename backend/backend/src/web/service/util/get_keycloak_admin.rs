use keycloak::{KeycloakAdmin, KeycloakAdminToken, KeycloakError};
use crate::config::keycloak::KeycloakConfig;
use std::fmt;

pub async fn get_keycloak_admin(keycloak_config: &KeycloakConfig) -> Result<KeycloakAdmin, KeycloakError> {
    log::info!("{:?}", LogEvents::GetKeyCloakAdminStart);
    let client = reqwest::Client::new();
    let admin_token =
        KeycloakAdminToken::acquire(&keycloak_config.url,
                                    &keycloak_config.username,
                                    &keycloak_config.password,
                                    &client).await;
    if admin_token.is_err() {
        let error = admin_token.err().unwrap();
        log::error!("{:?} {:?}",
                        LogEvents::GetKeyCloakAdminError,
                        error);
        return Err(error)
    }

    let admin_token = admin_token.unwrap();

    log::info!("{:?}", LogEvents::GetKeyCloakAdminOk);
    Ok(KeycloakAdmin::new(keycloak_config.url.as_str(), admin_token, client))
}

#[derive(Debug)]
enum LogEvents {
    GetKeyCloakAdminStart,
    GetKeyCloakAdminError,
    GetKeyCloakAdminOk,
}

impl fmt::Display for LogEvents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogEvents::GetKeyCloakAdminStart => write!(f, "GetKeyCloakAdminStart"),
            LogEvents::GetKeyCloakAdminError => write!(f, "GetKeyCloakAdminError"),
            LogEvents::GetKeyCloakAdminOk => write!(f, "GetKeyCloakAdminOk"),
        }
    }
}
