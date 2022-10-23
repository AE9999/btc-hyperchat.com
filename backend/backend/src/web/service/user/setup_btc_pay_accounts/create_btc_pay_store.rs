use actix_web::http::StatusCode;
use actix_web::web::Data;
use btcpay_rust::apis::stores_api::stores_create_store;
use btcpay_rust::models::{StoreBaseData, StoreData};
use uuid::Uuid;
use crate::AppContext;
use std::fmt;

pub async fn create_btc_pay_store(keycloak_id: &Uuid,
                              context: &Data<AppContext>) -> Result<StoreData,
                                                                    (StatusCode, String)> {
    log::info!("{:?}, {:?}",
                LogEvents::CreateBtcPayStoreStart,
                keycloak_id);


    // Create a store in BTC_PAY & Store in DB
    let mut store_base_data = StoreBaseData::new();
    store_base_data.name = Some(format!("Store-for-{}", keycloak_id.to_string()));
    let store_data =
        stores_create_store(&context.clients.btc_pay_configuration,
                            store_base_data).await;

    if store_data.is_err() {
        log::error!("{:?}, {:?}, {:?}",
                    LogEvents::CreateBtcPayStoreError,
                    store_data.as_ref().err().unwrap().to_string(),
                    &context.clients.btc_pay_configuration);
        return Err((StatusCode::INTERNAL_SERVER_ERROR,
                    LogEvents::CreateBtcPayStoreError.to_string()))
    }

    log::info!("{:?}",
                LogEvents::CreateBtcPayStoreOk);

    Ok(store_data.unwrap())
}


#[derive(Debug)]
enum LogEvents {
    CreateBtcPayStoreStart,
    CreateBtcPayStoreError,
    CreateBtcPayStoreOk,
}

impl fmt::Display for LogEvents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogEvents::CreateBtcPayStoreStart
            => write!(f, "CreateBtcPayStoreStart"),
            LogEvents::CreateBtcPayStoreError
            => write!(f, "CreateBtcPayStoreError"),
            LogEvents::CreateBtcPayStoreOk
            => write!(f, "CreateBtcPayStoreOk"),
        }
    }
}
