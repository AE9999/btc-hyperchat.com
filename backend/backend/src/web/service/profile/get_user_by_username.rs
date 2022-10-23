use crate::AppContext;
use actix_web::http::StatusCode;
use crate::data::model::user::User;
use std::fmt;
use actix_web::web::Data;

pub fn get_user_by_username(username: &str,
                               context: &Data<AppContext>) -> Result<User, (StatusCode, String)> {

    log::info!("{:?} {:?}",
                username,
                LogEvents::GetUserByUsernameStart);

    let user =
        context.repositories
               .user_repository
               .find_by_username_and_is_active(username);

    if user.is_err() {
        log::error!("{:?} {:?}",
                   LogEvents::GetUserByUsernameError,
                   user.err().unwrap().to_string());
        return Err((StatusCode::INTERNAL_SERVER_ERROR,
                    LogEvents::GetUserByUsernameError.to_string()));
    }

    let user = user.unwrap();

    if user.is_none() {
        log::error!("{:?} {:?}", LogEvents::GetUserByUsernameNonExistingUser, username);
        return Err((StatusCode::BAD_REQUEST,
                    LogEvents::GetUserByUsernameNonExistingUser.to_string()));
    }

    let user = user.unwrap();

    log::info!("{:?}",
                LogEvents::GetUserByUsernameOk);

    Ok(user)
}

#[derive(Debug)]
enum LogEvents {
    GetUserByUsernameStart,
    GetUserByUsernameError,
    GetUserByUsernameNonExistingUser,
    GetUserByUsernameOk,
}

impl fmt::Display for LogEvents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogEvents::GetUserByUsernameStart
                => write!(f, "GetUserByUsernameStart"),
            LogEvents::GetUserByUsernameError
                => write!(f, "GetUserByUsernameError"),
            LogEvents::GetUserByUsernameNonExistingUser
                => write!(f, "GetUserByUsernameNonExistingUser"),
            LogEvents::GetUserByUsernameOk
                => write!(f, "GetUserByUsernameOk"),
        }
    }
}
