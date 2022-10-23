use actix_web::http::StatusCode;
use crate::AppContext;
use uuid::Uuid;
use crate::data::model::user::User;
use std::fmt;
use actix_web::web::Data;

pub fn create_user_in_db_from_username_and_keycloak_id(username: &str,
                                                       email: &str,
                                                       keycloak_id: &Uuid,
                                                       context: &Data<AppContext>) -> Result<User, (StatusCode, String)> {

    log::info!("{:?}, {:?}, {:?}",
                LogEvents::CreateUserInDbFromUsernameAndKeycloakIdStart,
                username,
                keycloak_id);

    let initial_btc_password = Uuid::new_v4().to_string();

    // Compile User & Store it in DB.
    let user  =
        context.repositories
               .user_repository
               .create(String::from(username),
                                            String::from(email),
                             initial_btc_password,
                                            keycloak_id.clone(),
                                                  true);
    if user.is_err() {
        log::error!("{:?}, {:?}",
                    LogEvents::CreateUserInDbFromUsernameAndKeycloakIdError,
                    user.as_ref().err().unwrap().to_string());
        return Err((StatusCode::INTERNAL_SERVER_ERROR,
                    LogEvents::CreateUserInDbFromUsernameAndKeycloakIdError.to_string()));
    }

    log::info!("{:?}",
                LogEvents::CreateUserInDbFromUsernameAndKeycloakIdOk);

    Ok(user.unwrap())
}


#[derive(Debug)]
enum LogEvents {
    CreateUserInDbFromUsernameAndKeycloakIdStart,
    CreateUserInDbFromUsernameAndKeycloakIdError,
    CreateUserInDbFromUsernameAndKeycloakIdOk,
}

impl fmt::Display for LogEvents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogEvents::CreateUserInDbFromUsernameAndKeycloakIdStart
                => write!(f, "CreateUserInDbFromUsernameAndKeycloakIdStart"),
            LogEvents::CreateUserInDbFromUsernameAndKeycloakIdError
                => write!(f, "CreateUserInDbFromUsernameAndKeycloakIdError"),
            LogEvents::CreateUserInDbFromUsernameAndKeycloakIdOk
                => write!(f, "CreateUserInDbFromUsernameAndKeycloakIdOk"),
        }
    }
}

