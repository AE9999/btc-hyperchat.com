use actix_web::http::StatusCode;
use std::fmt;
use actix_web::web::Data;
use crate::AppContext;
use crate::data::model::user::User;

pub fn delete_user_from_database(user:&mut User,
                                 context: &Data<AppContext>) -> Result<(), (StatusCode, String)> {

    log::info!("{:?}, {:?}",
                LogEvents::DeleteUserFromDatabaseStart,
                user);

    user.active = false;
    let ok = context.repositories.user_repository.update(user);
    if ok.is_err() {
        log::error!("{:?}, {:?}",
                    LogEvents::DeleteUserFromDatabaseError,
                    ok.err().unwrap().to_string());
        return Err((StatusCode::INTERNAL_SERVER_ERROR,
                    LogEvents::DeleteUserFromDatabaseError.to_string()));
    }

    log::info!("{:?}",
                LogEvents::DeleteUserFromDatabaseOk);

    Ok(())
}


#[derive(Debug)]
enum LogEvents {
    DeleteUserFromDatabaseStart,
    DeleteUserFromDatabaseError,
    DeleteUserFromDatabaseOk,
}

impl fmt::Display for LogEvents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogEvents::DeleteUserFromDatabaseStart
                => write!(f, "DeleteUserFromDatabaseStart"),
            LogEvents::DeleteUserFromDatabaseError
                => write!(f, "DeleteUserFromDatabaseError"),
            LogEvents::DeleteUserFromDatabaseOk
                => write!(f, "DeleteUserFromDatabaseOk"),
        }
    }
}
