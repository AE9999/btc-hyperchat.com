use actix_web::http::StatusCode;
use uuid::Uuid;
use crate::web::model::user::create_user_request::CreateUserRequest;
use std::str::FromStr;
use std::fmt;
use actix_web::web::Json;


pub fn parse_keycloak_id_from_create_user_request(body: &Json<CreateUserRequest>) -> Result<Uuid,
                                                                                           (StatusCode, String)> {

    log::info!("{:?}, {:?}",
                LogEvents::ParseKeyCloakIdFromCreateUserRequestStart,
                body);

    let keycloak_id = uuid::Uuid::from_str(&body.keycloak_id);

    if keycloak_id.is_err() {

        log::error!("{:?}, {:?}",
                    LogEvents::ParseKeyCloakIdFromCreateUserRequestError,
                    keycloak_id.err().unwrap().to_string());

        return Err((StatusCode::INTERNAL_SERVER_ERROR,
                    LogEvents::ParseKeyCloakIdFromCreateUserRequestError.to_string()))
    }

    let keycloak_id = keycloak_id.unwrap();

    log::info!("{:?}",
                LogEvents::ParseKeyCloakIdFromCreateUserRequestOk);

    Ok(keycloak_id)
}


#[derive(Debug)]
enum LogEvents {
    ParseKeyCloakIdFromCreateUserRequestStart,
    ParseKeyCloakIdFromCreateUserRequestError,
    ParseKeyCloakIdFromCreateUserRequestOk,
}

impl fmt::Display for LogEvents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogEvents::ParseKeyCloakIdFromCreateUserRequestStart
                => write!(f, "ParseKeyCloakIdFromCreateUserRequestStart"),
            LogEvents::ParseKeyCloakIdFromCreateUserRequestError
                => write!(f, "ParseKeyCloakIdFromCreateUserRequestError"),
            LogEvents::ParseKeyCloakIdFromCreateUserRequestOk
                => write!(f, "ParseKeyCloakIdFromCreateUserRequestOk"),
        }
    }
}
