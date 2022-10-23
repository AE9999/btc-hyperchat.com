use actix_web::http::StatusCode;
use crate::data::model::user::User;
use std::fmt;
use actix_web::web::Data;
use crate::AppContext;
use crate::data::model::store::Store;

pub fn find_owner_for_store(store: &Store,
                            context: &Data<AppContext>) -> Result<User,
                                        (StatusCode, String)> {
    log::info!("{:?} {:?}",
                LogEvents::FindOwnerForStoreStart,
                store);

    let user =
        context.repositories
            .user_repository
            .find_by_pk_and_is_active(&store.owner_id);

    if user.is_err() {

        log::error!("{:?} {:?}",
                    LogEvents::FindOwnerForStoreError,
                    user.as_ref().err().unwrap().to_string());
        return Err((StatusCode::INTERNAL_SERVER_ERROR,
                    LogEvents::FindOwnerForStoreError.to_string()))
    }

    let user = user.unwrap();

    if user.is_none() {
        log::error!("{:?}",
                    LogEvents::FindOwnerForStoreNoUser);
        return Err((StatusCode::BAD_REQUEST,
                    LogEvents::FindOwnerForStoreNoUser.to_string()));
    }

    let user = user.unwrap();

    log::info!("{:?}",
                LogEvents::FindOwnerForStoreOk);

    Ok(user)
}


#[derive(Debug)]
enum LogEvents {
    FindOwnerForStoreStart,
    FindOwnerForStoreError,
    FindOwnerForStoreNoUser,
    FindOwnerForStoreOk,
}

impl fmt::Display for LogEvents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogEvents::FindOwnerForStoreStart => write!(f, "FindOwnerForStoreStart"),
            LogEvents::FindOwnerForStoreError => write!(f, "FindStoreForeBtcChatError"),
            LogEvents::FindOwnerForStoreNoUser => write!(f, "FindOwnerForStoreNoUser"),
            LogEvents::FindOwnerForStoreOk => write!(f, "FindOwnerForStoreOk"),
        }
    }
}
