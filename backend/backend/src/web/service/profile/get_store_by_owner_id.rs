use uuid::Uuid;
use crate::AppContext;
use actix_web::http::StatusCode;
use crate::data::model::store::Store;
use std::fmt;
use actix_web::web::Data;

pub fn get_store_by_owner_id(owner_id: &Uuid,
                             context: &Data<AppContext>) -> Result<Store, (StatusCode, String)> {

    log::info!("{:?}, {:?}",
                LogEvents::GetStoreByOwnerIdStart, owner_id);

    let stores =
        context.repositories.store_repository
            .find_by_owner_id(owner_id);

    if stores.is_err() {
        log::error!("{:?} {:?}",
                    LogEvents::GetStoreByOwnerIdDbFailure,
                    stores.err().unwrap().to_string());
        return Err((StatusCode::INTERNAL_SERVER_ERROR,
                    LogEvents::GetStoreByOwnerIdDbFailure.to_string()));
    }

    let mut stores = stores.unwrap();

    if stores.is_empty() {
        log::error!("{:?} {:?}",
                    LogEvents::GetStoreByOwnerIdNonExistingStore,
                    owner_id);
        return Err((StatusCode::BAD_REQUEST,
                    LogEvents::GetStoreByOwnerIdNonExistingStore.to_string()));
    }

    if stores.len() > 1 {
        log::error!("{:?}",
                    LogEvents::GetStoreByOwnerIdTooManyStores);
        return Err((StatusCode::BAD_REQUEST,
                    LogEvents::GetStoreByOwnerIdTooManyStores.to_string()));
    }

    log::info!("{:?}",
                LogEvents::GetStoreByOwnerIdOk);

    Ok(stores.remove(0))
}

#[derive(Debug)]
enum LogEvents {
    GetStoreByOwnerIdStart,
    GetStoreByOwnerIdDbFailure,
    GetStoreByOwnerIdNonExistingStore,
    GetStoreByOwnerIdTooManyStores,
    GetStoreByOwnerIdOk,
}

impl fmt::Display for LogEvents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogEvents::GetStoreByOwnerIdStart
                => write!(f, "GetStoreByOwnerIdStart"),
            LogEvents::GetStoreByOwnerIdDbFailure
                => write!(f, "GetStoreByOwnerIdDbFailure"),
            LogEvents::GetStoreByOwnerIdNonExistingStore
                => write!(f, "GetStoreByOwnerIdNonExistingStore"),
            LogEvents::GetStoreByOwnerIdTooManyStores
                => write!(f, "GetStoreByOwnerIdTooManyStores"),
            LogEvents::GetStoreByOwnerIdOk
                => write!(f, "GetStoreByOwnerIdOk"),
        }
    }
}
