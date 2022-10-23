use actix_web::HttpResponse;
use actix_web::http::StatusCode;
use actix_web::web::Data;
use crate::AppContext;
use std::fmt;
use actix_web_middleware_keycloak_auth::StandardKeycloakClaims;
use crate::web::model::settings::store_webhook_activation::store_webhook_activation::StoreWebhookActivation;
use crate::web::service::btc_chat::my_received_btc_chats::find_user_by_keycloak_id::find_user_by_keycloak_id;
use crate::web::service::btc_chat::util::find_store_for_user_id::find_store_for_owner_id;

pub async fn get_store_webhook_activation(claims: StandardKeycloakClaims,
                                          context: Data<AppContext>) -> HttpResponse {

    log::info!("{:?}",
               LogEvents::GetStoreWebhookActivationStart);

    let result =
        do_get_store_webhook_activation(&claims,
                                       &context).await;
    match result {
        Ok(store_webhook_activation) => {
            log::info!("{:?}",
                        LogEvents::GetStoreWebhookActivationOk);
            HttpResponse::Ok().json(store_webhook_activation)
        },
        Err(error) => {
            log::error!("{:?} {:?} ",
                        LogEvents::GetStoreWebhookActivationError,
                        error.1.clone());
            HttpResponse::build(error.0.clone())
                .body(LogEvents::GetStoreWebhookActivationError.to_string())
        }
    }
}

async fn do_get_store_webhook_activation(claims: &StandardKeycloakClaims,
                                         context: &Data<AppContext>)
    -> Result<StoreWebhookActivation,
              (StatusCode, String)> {

    let user =
        find_user_by_keycloak_id(&claims.sub,
                                 context)?;

    let store =
        find_store_for_owner_id(&user.id,
                                context)?;

    let store_webhook_activation =
        StoreWebhookActivation::new(store.webhook_active,
                                    store.automatically_process_btc_chats_if_webhook_succeeds);

    Ok(store_webhook_activation)
}

#[derive(Debug)]
pub enum LogEvents {
    GetStoreWebhookActivationStart,
    GetStoreWebhookActivationError,
    GetStoreWebhookActivationOk,
}

impl fmt::Display for LogEvents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogEvents::GetStoreWebhookActivationStart
            => write!(f, "GetStoreWebhookActivationStart"),
            LogEvents::GetStoreWebhookActivationError
            => write!(f, "GetStoreWebhookActivationError"),
            LogEvents::GetStoreWebhookActivationOk
            => write!(f, "GetStoreWebhookActivationOk"),
        }
    }
}
