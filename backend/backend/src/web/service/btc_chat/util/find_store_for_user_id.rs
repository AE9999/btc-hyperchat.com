use uuid::Uuid;
use actix_web::http::StatusCode;
use std::fmt;
use actix_web::web::Data;
use crate::AppContext;
use crate::data::model::store::Store;


pub fn find_store_for_owner_id(owner_id: &Uuid,
                               context: &Data<AppContext>) -> Result<Store,
                                                                  (StatusCode, String)> {
    log::info!("{:?} {:?}",
                LogEvents::FindStoreForeOwnerStart,
                owner_id);

    let store =
        context.repositories
            .store_repository
            .find_by_owner_id(owner_id);

    if store.is_err() {

        log::error!("{:?} {:?}",
                    LogEvents::FindStoreForeOwnerError,
                    store.as_ref().err().unwrap().to_string());
        return Err((StatusCode::INTERNAL_SERVER_ERROR,
                    LogEvents::FindStoreForeOwnerError.to_string()))
    }

    let mut stores = store.unwrap();

    // TODO fix stuff with multiple stores
    if stores.len() != 1 {
        log::error!("{:?} {:?}",
                    LogEvents::FindStoreForeOwnerIncorrectNumberOfStores,
                    stores.len());
        return Err((StatusCode::BAD_REQUEST,
                    LogEvents::FindStoreForeOwnerIncorrectNumberOfStores.to_string()));
    }

    let store = stores.remove(0);

    log::info!("{:?} {:?}",
                LogEvents::FindStoreForeOwnerOk,
                owner_id);

    Ok(store)
}

#[derive(Debug)]
enum LogEvents {
    FindStoreForeOwnerStart,
    FindStoreForeOwnerError,
    FindStoreForeOwnerIncorrectNumberOfStores,
    FindStoreForeOwnerOk,
}

impl fmt::Display for LogEvents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogEvents::FindStoreForeOwnerStart => write!(f, "FindStoreForeOwnerStart"),
            LogEvents::FindStoreForeOwnerError => write!(f, "FindStoreForeOwnerError"),
            LogEvents::FindStoreForeOwnerIncorrectNumberOfStores => write!(f, "FindStoreForeOwnerIncorrectNumberOfStores"),
            LogEvents::FindStoreForeOwnerOk => write!(f, "FindStoreForeOwnerOk"),
        }
    }
}

