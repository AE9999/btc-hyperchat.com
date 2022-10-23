use actix_web::HttpResponse;
use actix_web::http::StatusCode;
use actix_web::web::Data;
use actix_web::web;
use crate::AppContext;
use std::fmt;
use crate::web::model::profile::find_profile_names_response::FindProfileNamesResponse;

pub async fn find_profiles(path: web::Path<String>,
                           context: Data<AppContext>) -> HttpResponse {
    let username_prefix = path.into_inner();
    log::info!("{:?} {:?} ",
               LogEvents::FindProfileNamesStart,
               username_prefix);

    let result =
        do_find_profiles(&username_prefix,
                             &context).await;

    match result {
        Ok(profile) => {
            log::info!("{:?} {:?} ",
                        LogEvents::FindProfileNamesOk, profile);
            HttpResponse::Ok().json(profile)
        },
        Err(error) => {
            log::error!("{:?} {:?} ",
                        LogEvents::FindProfileNamesError, error.1.clone());
            HttpResponse::build(error.0.clone())
                .body(LogEvents::FindProfileNamesError.to_string())
        }
    }
}

async fn do_find_profiles(username_prefix: &String,
                          context: &Data<AppContext>) -> Result<FindProfileNamesResponse, (StatusCode, String)> {
    let users =
        context.repositories.user_repository
              .find_like_username_and_is_active_with_limit(username_prefix,
                                                           context.config
                                                         .general_config
                                                         .profile_autocomplete_max_results);

    if users.is_err() {
        log::error!("{:?} {:?}",
                    LogEvents::FindProfileNamesError,
                    users.as_ref().err().unwrap().to_string());
        return Err((StatusCode::INTERNAL_SERVER_ERROR,
                    LogEvents::FindProfileNamesError.to_string()))
    }

    let users = users.unwrap();

    let profile_names:Vec<String> =
        users.iter()
             .map(|user| user.username.clone())
             .collect();

    Ok(FindProfileNamesResponse::new(profile_names))
}

#[derive(Debug)]
pub enum LogEvents {
    FindProfileNamesStart,
    FindProfileNamesError,
    FindProfileNamesOk,
}

impl fmt::Display for LogEvents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogEvents::FindProfileNamesStart
                => write!(f, "FindProfileNamesStart"),
            LogEvents::FindProfileNamesError
                => write!(f, "FindProfileNamesError"),
            LogEvents::FindProfileNamesOk
                => write!(f, "FindProfileNamesOk"),
        }
    }
}
