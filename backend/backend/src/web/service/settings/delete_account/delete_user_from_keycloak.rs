use actix_web::http::StatusCode;
use std::fmt;
use actix_web::web::Data;
use uuid::Uuid;
use crate::AppContext;
use crate::web::service::util::get_keycloak_admin::get_keycloak_admin;

pub async fn delete_user_from_keycloak(keycloak_id: &Uuid,
                                      context: &Data<AppContext>) -> Result<(), (StatusCode, String)> {

    log::info!("{:?}, {:?}",
                LogEvents::DeleteUserFromKeyCloakStart,
                keycloak_id);

    let keycloak_admin =
        get_keycloak_admin(&context.config.keycloak_config).await;

    if keycloak_admin.is_err() {
        log::error!("{:?}, {:?}",
                    LogEvents::DeleteUserFromKeyCloakKeyCloakAdminError,
                    keycloak_admin.err().unwrap().to_string());
        return Err((StatusCode::INTERNAL_SERVER_ERROR,
                    LogEvents::DeleteUserFromKeyCloakKeyCloakAdminError.to_string()));
    }

    let keycloak_admin = keycloak_admin.unwrap();

    let ok =
        keycloak_admin.realm_users_with_id_delete(&context.config.keycloak_config.realm,
                                                  keycloak_id.to_string().as_str())
                      .await;

    if ok.is_err() {
        log::error!("{:?}, {:?}",
                    LogEvents::DeleteUserFromKeyCloakKeyCloakDeleteError,
                    ok.err().unwrap().to_string());
        return Err((StatusCode::INTERNAL_SERVER_ERROR,
                    LogEvents::DeleteUserFromKeyCloakKeyCloakDeleteError.to_string()));
    }

    log::info!("{:?}",
                LogEvents::DeleteUserFromKeyCloakOk);

    Ok(())
}

#[derive(Debug)]
enum LogEvents {
    DeleteUserFromKeyCloakStart,
    DeleteUserFromKeyCloakKeyCloakAdminError,
    DeleteUserFromKeyCloakKeyCloakDeleteError,
    DeleteUserFromKeyCloakOk,
}

impl fmt::Display for LogEvents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogEvents::DeleteUserFromKeyCloakStart
                => write!(f, "DeleteUserFromKeyCloakStart"),
            LogEvents::DeleteUserFromKeyCloakKeyCloakAdminError
                => write!(f, "DeleteUserFromKeyCloakKeyCloakAdminError"),
            LogEvents::DeleteUserFromKeyCloakKeyCloakDeleteError
                => write!(f, "DeleteUserFromKeyCloakKeyCloakDeleteError"),
            LogEvents::DeleteUserFromKeyCloakOk
                => write!(f, "DeleteUserFromKeyCloakOk"),
        }
    }
}
