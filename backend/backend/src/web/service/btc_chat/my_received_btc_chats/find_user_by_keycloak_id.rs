use actix_web::http::StatusCode;
use std::fmt;
use actix_web::web::Data;
use crate::AppContext;
use uuid::Uuid;
use crate::data::model::user::User;

pub fn find_user_by_keycloak_id(keycloak_id: &Uuid,
                                context: &Data<AppContext>) -> Result<User, (StatusCode, String)> {
    log::info!("{:?} {:?}",
                LogEvents::FindUserByKeycloakIdStart,
                keycloak_id);

    let user =
        context.repositories
            .user_repository
            .find_by_keycloak_id_and_is_active(keycloak_id);

    if user.is_err() {
        log::error!("{:?} {:?}",
                    LogEvents::FindUserByKeycloakIdError,
                    user.as_ref().err().unwrap().to_string());
        return Err((StatusCode::INTERNAL_SERVER_ERROR,
                    LogEvents::FindUserByKeycloakIdError.to_string()))
    }

    let user = user.unwrap();

    if user.is_none() {
        log::error!("{:?}",
                    LogEvents::FindUserByKeycloakNoneFound);
        return Err((StatusCode::BAD_REQUEST,
                    LogEvents::FindUserByKeycloakNoneFound.to_string()))
    }

    let user = user.unwrap();

    log::info!("{:?}",
                LogEvents::FindUserByKeycloakIdOk);

    Ok(user)

}

#[derive(Debug)]
enum LogEvents {
    FindUserByKeycloakIdStart,
    FindUserByKeycloakIdError,
    FindUserByKeycloakNoneFound,
    FindUserByKeycloakIdOk,
}

impl fmt::Display for LogEvents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogEvents::FindUserByKeycloakIdStart => write!(f, "FindUserByKeycloakIdStart"),
            LogEvents::FindUserByKeycloakIdError => write!(f, "FindUserByKeycloakIdError"),
            LogEvents::FindUserByKeycloakNoneFound => write!(f, "FindUserByKeycloakNoneFound"),
            LogEvents::FindUserByKeycloakIdOk => write!(f, "FindUserByKeycloakIdOk"),
        }
    }
}
