use actix_web_httpauth::extractors::basic::BasicAuth;
use actix_web::{Error, dev::ServiceRequest};
use actix_web::error::ErrorUnauthorized;
use actix_web::web::Data;
use crate::AppContext;

// Borrowed from https://users.rust-lang.org/t/actix-log-in-authentication/47423

#[derive(Debug)]
enum LogEvents {
    StartAuthValidation,
    FailureAuthValidation,
    SuccessAuthValidation,
}

pub async fn basic_auth_validator(req: ServiceRequest,
                              credentials: BasicAuth) -> Result<ServiceRequest, (Error, ServiceRequest)>
{
    log::info!("{:?} {:?} ", LogEvents::StartAuthValidation, credentials);
    let context: &AppContext = req.app_data::<Data<AppContext>>()
                               .expect("Context unavailable");
    if credentials.user_id().eq(&(context.config.general_config.user_registration_username))
        && credentials.password().is_some()
        && credentials.password().unwrap() == context.config.general_config.user_registration_password {
        log::info!("{:?}", LogEvents::SuccessAuthValidation);
        Ok(req)
    } else {
        log::info!("{:?} {:?} {:?}", LogEvents::FailureAuthValidation, credentials.user_id(), credentials.password());
        Err((ErrorUnauthorized("unauthorized"), req))
    }
}
