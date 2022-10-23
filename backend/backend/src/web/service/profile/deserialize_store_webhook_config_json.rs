use std::fmt;
use actix_web::http::StatusCode;
use crate::web::model::settings::store_webhook_config::store_webhook_config::StoreWebhookConfig;

pub fn deserialize_store_webhook_config_json(store_webhook_config_json: &str)
                                             -> Result<StoreWebhookConfig,
                                                       (StatusCode, String)> {

    log::info!("{:?}, {:?}",
                LogEvents::DeserializeStoreWebhookConfigJsonStart,
                store_webhook_config_json);

    let store_webhook_config : serde_json::Result<StoreWebhookConfig> =
        serde_json::from_str(store_webhook_config_json);

    if store_webhook_config.is_err() {

        log::error!("{:?} {:?}",
                    LogEvents::DeserializeStoreWebhookConfigJsonError,
                    store_webhook_config.as_ref().err().unwrap().to_string());
        return Err((StatusCode::INTERNAL_SERVER_ERROR,
                    LogEvents::DeserializeStoreWebhookConfigJsonError.to_string()))
    }

    let store_webhook_config =
        store_webhook_config.unwrap();

    log::info!("{:?}",
                LogEvents::DeserializeWebhookConfigJsonOk);

    Ok(store_webhook_config)
}

#[derive(Debug)]
pub enum LogEvents {
    DeserializeStoreWebhookConfigJsonStart,
    DeserializeStoreWebhookConfigJsonError,
    DeserializeWebhookConfigJsonOk,
}

impl fmt::Display for LogEvents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogEvents::DeserializeStoreWebhookConfigJsonStart
                => write!(f, "DeserializeStoreWebhookConfigJsonStart"),
            LogEvents::DeserializeStoreWebhookConfigJsonError
                => write!(f, "DeserializeStoreWebhookConfigJsonError"),
            LogEvents::DeserializeWebhookConfigJsonOk
                => write!(f, "DeserializeWebhookConfigJsonOk"),
        }
    }
}
