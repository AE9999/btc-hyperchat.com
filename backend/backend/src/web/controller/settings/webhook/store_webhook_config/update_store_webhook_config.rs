use actix_web::HttpResponse;
use actix_web::http::StatusCode;
use actix_web::web::{Data, Json};
use crate::AppContext;
use std::fmt;
use actix_web_middleware_keycloak_auth::StandardKeycloakClaims;
use crate::web::model::settings::store_webhook_config::store_webhook_config::StoreWebhookConfig;
use crate::web::service::btc_chat::my_received_btc_chats::find_user_by_keycloak_id::find_user_by_keycloak_id;
use crate::web::service::btc_chat::util::find_store_for_user_id::find_store_for_owner_id;
use crate::web::service::profile::serialize_store_webhook_config::serialize_store_webhook_config;

pub async fn update_store_webhook_config(body: Json<StoreWebhookConfig>,
                                         claims: StandardKeycloakClaims,
                                         context: Data<AppContext>) -> HttpResponse {

    log::info!("{:?} {:?} ",
               LogEvents::UpdateStoreWebhookConfigStart,
               body);

    let result = do_update_settings(body.0,
                                    &claims,
                                    &context).await;
    match result {
        Ok(_) => {
            log::info!("{:?}",
                        LogEvents::UpdateStoreWebhookConfigOk);
            HttpResponse::Ok().finish()
        },
        Err(error) => {
            log::error!("{:?} {:?} ",
                        LogEvents::UpdateStoreWebhookConfigError,
                        error.1.clone());
            HttpResponse::build(error.0.clone())
                .body(LogEvents::UpdateStoreWebhookConfigError.to_string())
        }
    }
}

async fn do_update_settings(store_webhook_config: StoreWebhookConfig,
                            claims: &StandardKeycloakClaims,
                            context: &Data<AppContext>) -> Result<(), (StatusCode, String)> {

    let user =
        find_user_by_keycloak_id(&claims.sub,
                                 context)?;

    let mut store =
        find_store_for_owner_id(&user.id,
                                context)?;

    if !store_webhook_config.is_correct() {
        log::info!("{:?}",
                LogEvents::UpdateStoreWebhookConfigDataFailure);

        return Err((StatusCode::BAD_REQUEST,
                    LogEvents::UpdateStoreWebhookConfigDataFailure.to_string()))
    }

    if store_webhook_config.is_strictly_empty() {
        store.webhook_active =
            false;
        store.automatically_process_btc_chats_if_webhook_succeeds =
            false;
    }

    store.webhook_config_json =
        serialize_store_webhook_config(store_webhook_config)?;

    let ok =
        context.repositories.store_repository
            .update(&store);

    if ok.is_err() {

        log::error!("{:?} {:?}",
                LogEvents::UpdateStoreWebhookConfigDbFailure,
                ok.as_ref().err().unwrap().to_string());

        return Err((StatusCode::INTERNAL_SERVER_ERROR,
                    LogEvents::UpdateStoreWebhookConfigDbFailure.to_string()))
    }


    Ok(())
}

#[derive(Debug)]
pub enum LogEvents {
    UpdateStoreWebhookConfigStart,
    UpdateStoreWebhookConfigDataFailure,
    UpdateStoreWebhookConfigDbFailure,
    UpdateStoreWebhookConfigError,
    UpdateStoreWebhookConfigOk,
}

impl fmt::Display for LogEvents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogEvents::UpdateStoreWebhookConfigStart
                => write!(f, "UpdateStoreWebhookConfigStart"),
            LogEvents::UpdateStoreWebhookConfigDataFailure
                => write!(f, "UpdateStoreWebhookConfigDataFailure"),
            LogEvents::UpdateStoreWebhookConfigDbFailure
                => write!(f, "UpdateStoreWebhookConfigDbFailure"),
            LogEvents::UpdateStoreWebhookConfigError
                => write!(f, "UpdateStoreWebhookConfigError"),
            LogEvents::UpdateStoreWebhookConfigOk
                => write!(f, "UpdateStoreWebhookConfigOk"),
        }
    }
}
