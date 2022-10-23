use std::fmt;
use actix_web::http::StatusCode;
use crate::web::model::settings::store_webhook_config::store_webhook_config::StoreWebhookConfig;

pub fn serialize_store_webhook_config(store_webhook_config: StoreWebhookConfig) -> Result<String,
                                                                                    (StatusCode, String)> {

    log::info!("{:?}, {:?}",
                LogEvents::SerializeStoreWebhookConfigStart,
                store_webhook_config);

    let store_webhook_config  =
        serde_json::to_string(&store_webhook_config);

    if store_webhook_config.is_err() {

        log::error!("{:?} {:?}",
                    LogEvents::SerializeStoreWebhookConfigError,
                    store_webhook_config.as_ref().err().unwrap().to_string());
        return Err((StatusCode::INTERNAL_SERVER_ERROR,
                    LogEvents::SerializeStoreWebhookConfigError.to_string()))
    }

    let store_webhook_config = store_webhook_config.unwrap();

    log::info!("{:?}",
                LogEvents::SerializeStoreWebhookConfigOk);

    Ok(store_webhook_config)
}

#[derive(Debug)]
pub enum LogEvents {
    SerializeStoreWebhookConfigStart,
    SerializeStoreWebhookConfigError,
    SerializeStoreWebhookConfigOk,
}

impl fmt::Display for LogEvents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogEvents::SerializeStoreWebhookConfigStart
                => write!(f, "SerializeStoreWebhookConfigStart"),
            LogEvents::SerializeStoreWebhookConfigError
                => write!(f, "SerializeStoreWebhookConfigError"),
            LogEvents::SerializeStoreWebhookConfigOk
                => write!(f, "SerializeStoreWebhookConfigOk"),
        }
    }
}
