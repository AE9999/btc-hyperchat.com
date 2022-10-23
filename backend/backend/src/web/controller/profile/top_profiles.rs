use actix_web::HttpResponse;
use actix_web::http::StatusCode;
use actix_web::web::Data;
use crate::AppContext;
use std::fmt;
use crate::web::model::profile::find_profile_names_response::FindProfileNamesResponse;

pub async fn top_profiles(context: Data<AppContext>) -> HttpResponse {

    log::info!("{:?}",
               LogEvents::TopProfileNamesStart);

    let result =
        do_top_profiles(
                         &context).await;

    match result {
        Ok(profile) => {
            log::info!("{:?}",
                        LogEvents::TopProfileNamesOk);
            HttpResponse::Ok().json(profile)
        },
        Err(error) => {
            log::error!("{:?} {:?} ",
                        LogEvents::TopProfileNamesError, error.1.clone());
            HttpResponse::build(error.0.clone())
                .body(LogEvents::TopProfileNamesError.to_string())
        }
    }
}

async fn do_top_profiles(context: &Data<AppContext>) -> Result<FindProfileNamesResponse, (StatusCode, String)> {

    let top_profiles: Vec<&str> =
        context.config
               .general_config.top_profiles
               .split(",")
               .collect();

    let top_profiles =
        context.repositories.user_repository
               .find_with_username_in_and_is_active(&top_profiles);

    if top_profiles.is_err() {
        log::error!("{:?} {:?}",
                    LogEvents::TopProfileNamesError,
                    top_profiles.as_ref().err().unwrap().to_string());
        return Err((StatusCode::INTERNAL_SERVER_ERROR,
                    LogEvents::TopProfileNamesError.to_string()))
    }

    let top_profiles = top_profiles.unwrap();

    let profile_names:Vec<String> =
        top_profiles.iter()
                    .map(|user| user.username.clone())
                    .collect();

    Ok(FindProfileNamesResponse::new(profile_names))
}

#[derive(Debug)]
pub enum LogEvents {
    TopProfileNamesStart,
    TopProfileNamesError,
    TopProfileNamesOk,
}

impl fmt::Display for LogEvents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogEvents::TopProfileNamesStart
                => write!(f, "TopProfileNamesStart"),
            LogEvents::TopProfileNamesError
                => write!(f, "TopProfileNamesError"),
            LogEvents::TopProfileNamesOk
                => write!(f, "TopProfileNamesOk"),
        }
    }
}
