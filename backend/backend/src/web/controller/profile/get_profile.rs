use actix_web::HttpResponse;
use actix_web::http::StatusCode;
use actix_web::web::Data;
use actix_web::web;
use crate::AppContext;
use crate::web::model::profile::profile::Profile;
use crate::web::service::profile::get_store_by_owner_id::get_store_by_owner_id;
use std::fmt;
use crate::web::service::profile::get_user_by_username::get_user_by_username;

pub async fn get_profile(path: web::Path<String>,
                         context: Data<AppContext>) -> HttpResponse {
    let username = path.into_inner();
    log::info!("{:?} {:?} ",
               LogEvents::GetProfileStart,
               username);

    let result =
        do_get_profile(&username,
                       &context).await;

    match result {
        Ok(profile) => {
            log::info!("{:?} {:?} ",
                        LogEvents::GetProfileOk, profile);
            HttpResponse::Ok().json(profile)
        },
        Err(error) => {
            log::error!("{:?} {:?} ",
                        LogEvents::GetProfileError, error.1.clone());
            HttpResponse::build(error.0.clone())
                .body(LogEvents::GetProfileError.to_string())
        }
    }
}

async fn do_get_profile(username: &String,
                        context: &Data<AppContext>) -> Result<Profile, (StatusCode, String)> {

    // TODO(AE): Optimize the below two lines in a single query.

    let user = get_user_by_username(&username, context)?;

    let store = get_store_by_owner_id(&user.id, context)?;

    let currencies
        = context.config.invoice_config.accepted_currencies
                 .clone();

    Ok(Profile::new(username.clone(),
                            store.btcpay_store_id,
                            currencies))

}

#[derive(Debug)]
pub enum LogEvents {
    GetProfileStart,
    GetProfileError,
    GetProfileOk,
}

impl fmt::Display for LogEvents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogEvents::GetProfileStart
                => write!(f, "GetProfileStart"),
            LogEvents::GetProfileError
                => write!(f, "GetProfileError"),
            LogEvents::GetProfileOk
                => write!(f, "GetProfileOk"),
        }
    }
}

