use actix_web::http::StatusCode;
use crate::AppContext;
use uuid::Uuid;
use std::fmt;
use actix_web::web::Data;

pub fn validate_keycloak_id_does_not_exist(keycloak_id: &Uuid,
                                           context: &Data<AppContext>) -> Result<(), (StatusCode, String)> {

    log::info!("{:?}, {:?}",
                LogEvents::ValidateKeyCloakDoesNotExistStart,
                keycloak_id);

    // Query if there is an existing user
    let user =
        context.repositories.user_repository.find_by_keycloak_id_and_is_active(keycloak_id);

    if user.is_err() {
        let error = user.as_ref().err().unwrap();
        log::error!("{:?}, {:?}",
                    LogEvents::ValidateKeyCloakDoesNotExistDbFailure,
                    error);
        return Err((StatusCode::INTERNAL_SERVER_ERROR,
                    LogEvents::ValidateKeyCloakDoesNotExistDbFailure.to_string()));
    }

    let user = user.unwrap();

    if user.is_some() {
        log::info!("{:?}", LogEvents::ValidateKeyCloakDoesNotExistKeycloakIdAlreadyExistsFailure);
        return Err((StatusCode::BAD_REQUEST,
                    LogEvents::ValidateKeyCloakDoesNotExistKeycloakIdAlreadyExistsFailure.to_string()));
    }

    log::info!("{:?}",
                LogEvents::ValidateKeyCloakDoesNotExistOk);

    Ok(())
}

#[derive(Debug)]
enum LogEvents {
    ValidateKeyCloakDoesNotExistStart,
    ValidateKeyCloakDoesNotExistDbFailure,
    ValidateKeyCloakDoesNotExistKeycloakIdAlreadyExistsFailure,
    ValidateKeyCloakDoesNotExistOk,
}

impl fmt::Display for LogEvents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogEvents::ValidateKeyCloakDoesNotExistStart
                => write!(f, "ValidateKeyCloakDoesNotExistStart"),
            LogEvents::ValidateKeyCloakDoesNotExistDbFailure
                => write!(f, "ValidateKeyCloakDoesNotExistDbFailure"),
            LogEvents::ValidateKeyCloakDoesNotExistKeycloakIdAlreadyExistsFailure
                => write!(f, "ValidateKeyCloakDoesNotExistKeycloakIdAlreadyExistsFailure"),
            LogEvents::ValidateKeyCloakDoesNotExistOk
                => write!(f, "ValidateKeyCloakDoesNotExistOk"),
        }
    }
}



