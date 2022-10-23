use actix_web::HttpResponse;
use actix_web::http::StatusCode;
use actix_web::web::Data;
use crate::AppContext;
use std::fmt;
use actix_web_middleware_keycloak_auth::StandardKeycloakClaims;
use crate::web::model::settings::store_webhook_config::store_webhook_config::StoreWebhookConfig;
use crate::web::service::btc_chat::my_received_btc_chats::find_user_by_keycloak_id::find_user_by_keycloak_id;
use crate::web::service::btc_chat::util::find_store_for_user_id::find_store_for_owner_id;
use crate::web::service::profile::deserialize_store_webhook_config_json::deserialize_store_webhook_config_json;

pub async fn get_store_webhook_config(claims: StandardKeycloakClaims,
                                      context: Data<AppContext>) -> HttpResponse {

    log::info!("{:?} ",
               LogEvents::GetStoreWebhookConfigStart);

    let result =
        do_get_store_webhook_config(&claims, &context).await;

    match result {
        Ok(profile) => {
            log::info!("{:?} {:?} ",
                        LogEvents::GetStoreWebhookConfigOk, profile);
            HttpResponse::Ok().json(profile)
        },
        Err(error) => {
            log::error!("{:?} {:?} ",
                        LogEvents::GetStoreWebhookConfigError, error.1.clone());
            HttpResponse::build(error.0.clone())
                .body(LogEvents::GetStoreWebhookConfigError.to_string())
        }
    }
}

async fn do_get_store_webhook_config(claims: &StandardKeycloakClaims,
                                     context: &Data<AppContext>) -> Result<StoreWebhookConfig,
                                                               (StatusCode, String)> {

    let user =
        find_user_by_keycloak_id(&claims.sub,
                                context)?;

    let store =
        find_store_for_owner_id(&user.id,
                                context)?;

    let webhook_config =
        deserialize_store_webhook_config_json(store.webhook_config_json
                                                                       .as_str())?;


    Ok(webhook_config)
}

#[derive(Debug)]
pub enum LogEvents {
    GetStoreWebhookConfigStart,
    GetStoreWebhookConfigError,
    GetStoreWebhookConfigOk,
}

impl fmt::Display for LogEvents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogEvents::GetStoreWebhookConfigStart
                => write!(f, "GetStoreWebhookConfigStart"),
            LogEvents::GetStoreWebhookConfigError
                => write!(f, "GetStoreWebhookConfigError"),
            LogEvents::GetStoreWebhookConfigOk
                => write!(f, "GetStoreWebhookConfigOk"),
        }
    }
}
