use actix_web::http::StatusCode;
use crate::AppContext;
use std::fmt;
use actix_web::web::Data;

pub fn validate_username_does_not_exist(username: &str,
                                        context: &Data<AppContext>) -> Result<(), (StatusCode, String)> {

    log::info!("{:?}, {:?}",
                LogEvents::ValidateUsernameDoesNotExistStart,
                username);

    // Query if there is an existing user
    let user =
        context.repositories.user_repository.find_with_username_in_and_is_active(&vec!(username));

    if user.is_err() {
        let error = user.as_ref().err().unwrap();
        log::error!("{:?}, {:?}",
                    LogEvents::ValidateUsernameDoesNotExistDbFailure,
                    error);
        return Err((StatusCode::INTERNAL_SERVER_ERROR,
                    LogEvents::ValidateUsernameDoesNotExistDbFailure.to_string()));
    }

    let user = user.unwrap();

    if !user.is_empty() {
        log::info!("{:?}", LogEvents::ValidateUsernameDoesNotExistKeycloakIdAlreadyExistsFailure);
        return Err((StatusCode::BAD_REQUEST,
                    LogEvents::ValidateUsernameDoesNotExistKeycloakIdAlreadyExistsFailure.to_string()));
    }

    log::info!("{:?}",
                LogEvents::ValidateUsernameDoesNotExistOk);

    Ok(())
}

#[derive(Debug)]
enum LogEvents {
    ValidateUsernameDoesNotExistStart,
    ValidateUsernameDoesNotExistDbFailure,
    ValidateUsernameDoesNotExistKeycloakIdAlreadyExistsFailure,
    ValidateUsernameDoesNotExistOk,
}

impl fmt::Display for LogEvents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogEvents::ValidateUsernameDoesNotExistStart
            => write!(f, "ValidateUsernameDoesNotExistStart"),
            LogEvents::ValidateUsernameDoesNotExistDbFailure
            => write!(f, "ValidateUsernameDoesNotExistDbFailure"),
            LogEvents::ValidateUsernameDoesNotExistKeycloakIdAlreadyExistsFailure
            => write!(f, "ValidateUsernameDoesNotExistKeycloakIdAlreadyExistsFailure"),
            LogEvents::ValidateUsernameDoesNotExistOk
            => write!(f, "ValidateKeyCloakDoesNotExistOk"),
        }
    }
}
