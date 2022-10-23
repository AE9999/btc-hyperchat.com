use actix_web::HttpResponse;
use actix_web::http::StatusCode;
use actix_web::web::{Data, Json};
use crate::AppContext;
use std::fmt;
use actix_web_middleware_keycloak_auth::StandardKeycloakClaims;
use crate::web::model::settings::store_webhook_activation::store_webhook_activation::StoreWebhookActivation;
use crate::web::service::btc_chat::my_received_btc_chats::find_user_by_keycloak_id::find_user_by_keycloak_id;
use crate::web::service::btc_chat::util::find_store_for_user_id::find_store_for_owner_id;
use crate::web::service::profile::deserialize_store_webhook_config_json::deserialize_store_webhook_config_json;

pub async fn update_store_webhook_activation(body: Json<StoreWebhookActivation>,
                                             claims: StandardKeycloakClaims,
                                             context: Data<AppContext>) -> HttpResponse {

    log::info!("{:?} {:?} ",
               LogEvents::UpdateStoreWebhookActivationStart,
               body);

    let result =
        do_update_store_webhook_activation(body.0,
                                                  &claims,
                                                  &context).await;
    match result {
        Ok(_) => {
            log::info!("{:?}",
                        LogEvents::UpdateStoreWebhookActivationOk);
            HttpResponse::Ok().finish()
        },
        Err(error) => {
            log::error!("{:?} {:?} ",
                        LogEvents::UpdateStoreWebhookActivationError,
                        error.1.clone());
            HttpResponse::build(error.0.clone())
                         .body(LogEvents::UpdateStoreWebhookActivationError.to_string())
        }
    }
}

async fn do_update_store_webhook_activation(store_webhook_activation: StoreWebhookActivation,
                                            claims: &StandardKeycloakClaims,
                                            context: &Data<AppContext>) -> Result<(), (StatusCode, String)> {

    let user =
        find_user_by_keycloak_id(&claims.sub,
                                 context)?;

    let mut store =
        find_store_for_owner_id(&user.id,
                                context)?;

    let webhook_store_config =
        deserialize_store_webhook_config_json(store.webhook_config_json.as_str())?;

    if !webhook_store_config.has_minimally_needed_attributes()
        && store_webhook_activation.webhook_active {
        log::error!("{:?}",
                    LogEvents::UpdateStoreWebhookActivationNoWebHooksFailure);

        return Err((StatusCode::BAD_REQUEST,
                    LogEvents::UpdateStoreWebhookActivationNoWebHooksFailure.to_string()))
    }

    store.webhook_active =
        store_webhook_activation.webhook_active;
    store.automatically_process_btc_chats_if_webhook_succeeds =
        store_webhook_activation.automatically_process_btc_chats_if_webhook_succeeds;

    let ok =
        context.repositories.store_repository
               .update(&store);

    if ok.is_err() {

        log::info!("{:?} {:?}",
                LogEvents::UpdateStoreWebhookActivationDbFailure,
                ok.as_ref().err().unwrap().to_string());

        return Err((StatusCode::INTERNAL_SERVER_ERROR,
                    LogEvents::UpdateStoreWebhookActivationDbFailure.to_string()))
    }


    Ok(())
}

#[derive(Debug)]
pub enum LogEvents {
    UpdateStoreWebhookActivationStart,
    UpdateStoreWebhookActivationNoWebHooksFailure,
    UpdateStoreWebhookActivationDbFailure,
    UpdateStoreWebhookActivationError,
    UpdateStoreWebhookActivationOk,
}

impl fmt::Display for LogEvents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogEvents::UpdateStoreWebhookActivationStart
                => write!(f, "UpdateStoreWebhookActivationStart"),
            LogEvents::UpdateStoreWebhookActivationNoWebHooksFailure
                => write!(f, "UpdateStoreWebhookActivationDbFailure"),
            LogEvents::UpdateStoreWebhookActivationDbFailure
                => write!(f, "UpdateStoreWebhookActivationDbFailure"),
            LogEvents::UpdateStoreWebhookActivationError
                => write!(f, "UpdateStoreWebhookActivationError"),
            LogEvents::UpdateStoreWebhookActivationOk
                => write!(f, "UpdateStoreWebhookActivationOk"),
        }
    }
}
