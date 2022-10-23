use actix_web::HttpResponse;
use actix_web::http::StatusCode;
use actix_web::web::Data;
use actix_web::web::Json;
use crate::AppContext;
use crate::data::model::user::User;
use crate::web::model::user::create_user_request::CreateUserRequest;
use crate::web::service::user::create_user_in_db_from_username_and_keycloak_id::create_user_in_db_from_username_and_keycloak_id;
use crate::web::service::user::parse_keycloak_id_from_request::parse_keycloak_id_from_create_user_request;
use crate::web::service::user::setup_btc_pay_accounts::setup_btc_pay_accounts::setup_btc_pay_account;
use crate::web::service::user::validate_keycloak_id_does_not_exist::validate_keycloak_id_does_not_exist;
use std::fmt;
use crate::web::service::user::validate_username_does_not_exist::validate_username_does_not_exist;

pub async fn create_user(body: Json<CreateUserRequest>, context: Data<AppContext>) -> HttpResponse {
    log::info!("{:?} {:?} ",
               LogEvents::CreateUserStart,
               body.0);

    let result = do_create_user(body, context).await;
    match result {
        Ok(user) => {
            log::info!("{:?} {:?} ",
                       LogEvents::CreateUserOk, user);
            HttpResponse::Ok().finish()
        },
        Err(error) => {
            log::error!("{:?} {:?} ",
                        LogEvents::CreateUserError,
                        error.1.clone());
            HttpResponse::build(error.0.clone())
                         .body(LogEvents::CreateUserError.to_string())
        }
    }
}

async fn do_create_user(body: Json<CreateUserRequest>, context: Data<AppContext>) -> Result<User,
                                                                                  (StatusCode, String)>  {
    let keycloak_id = parse_keycloak_id_from_create_user_request(&body)?;

    validate_keycloak_id_does_not_exist(&keycloak_id,
                                        &context)?;

    validate_username_does_not_exist(body.username.as_str(),
                                        &context)?;

    let user =
        create_user_in_db_from_username_and_keycloak_id(body.username.as_str(),
                                                body.email.as_str(),
                                                &keycloak_id,
                                                       &context)?;

    setup_btc_pay_account(&user,
                          &keycloak_id,
                          &context).await?;

    Ok(user)
}

#[derive(Debug)]
pub enum LogEvents {
    CreateUserStart,
    CreateUserOk,
    CreateUserError,
}

impl fmt::Display for LogEvents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogEvents::CreateUserStart
                => write!(f, "CreateUserStart"),
            LogEvents::CreateUserOk
                => write!(f, "CreateUserOk"),
            LogEvents::CreateUserError
                => write!(f, "CreateUserError"),
        }
    }
}




