use actix_web::HttpResponse;
use actix_web::http::StatusCode;
use actix_web::web::Data;
use crate::AppContext;
use std::fmt;
use actix_web_middleware_keycloak_auth::StandardKeycloakClaims;
use chrono::{DateTime, Utc};
use crate::data::model::btc_chat::{BtcChat, ProcessingStatus};
use crate::web::model::settings::store_webhook_test_result::StoreWebhookTestResult;
use crate::web::service::btc_chat::my_received_btc_chats::find_user_by_keycloak_id::find_user_by_keycloak_id;
use crate::web::service::btc_chat::util::find_store_for_user_id::find_store_for_owner_id;
use crate::web::service::invoice::callback::webhook::make_callback_per_store_webhook_config::make_callback_per_store_webhook_config;
use crate::web::service::profile::deserialize_store_webhook_config_json::deserialize_store_webhook_config_json;

pub async fn test_store_webhook(claims: StandardKeycloakClaims,
                                context: Data<AppContext>) -> HttpResponse {

    log::info!("{:?}",
               LogEvents::TestWebhookStart);

    let result =
        do_test_webhook(&claims,
                        &context).await;
    match result {
        Ok(result) => {
            log::info!("{:?}",
                        LogEvents::TestWebhookOk);
            HttpResponse::Ok().json(result)
        },
        Err(error) => {
            log::error!("{:?} {:?} ",
                        LogEvents::TestWebhookError,
                        error.1.clone());
            HttpResponse::build(error.0.clone())
                .body(LogEvents::TestWebhookError.to_string())
        }
    }
}

async fn do_test_webhook(claims: &StandardKeycloakClaims,
                         context: &Data<AppContext>) -> Result<StoreWebhookTestResult, (StatusCode, String)> {

    let user =
        find_user_by_keycloak_id(&claims.sub,
                                 context)?;

    let store =
        find_store_for_owner_id(&user.id,
                                context)?;

    let store_webhook_config =
        deserialize_store_webhook_config_json(store.webhook_config_json
                                                                       .as_str())?;


    if !store_webhook_config.has_minimally_needed_attributes() {
        log::info!("{:?}",
                   LogEvents::TestWebhookNoWebhookConfigured);
        return Err((StatusCode::BAD_REQUEST,
                    LogEvents::TestWebhookNoWebhookConfigured.to_string()))
    }

    let now  = Utc::now();

    let mut btc_chat =
        create_test_btc_chat(context,
                             store.btcpay_store_id.clone(),
                             &now);

    let ok =
        make_callback_per_store_webhook_config(&store_webhook_config,
                                               &mut btc_chat,
                                               context).await;

    if ok.is_ok() {
        log::info!("{:?}",
                   LogEvents::TestWebhookTestOk);
    } else {
        log::info!("{:?}, {:?}",
                   LogEvents::TestWebhookTestFailed,
                   ok.as_ref().err().unwrap().1);
    }

    let webhook_test_result =
        StoreWebhookTestResult::new(ok.is_ok());

    Ok(webhook_test_result)
}

fn create_test_btc_chat(context: &Data<AppContext>,
                        store_id: String,
                        date: &DateTime<Utc>) -> BtcChat {
    BtcChat::new(context.config.test_webhook_config.id.clone(),
                 true,
                 ProcessingStatus::Confirmed.to_code(),
                  Some(context.config.test_webhook_config.message.clone()),
                 Some(context.config.test_webhook_config.sender.clone()),
                 store_id,
                 context.config.test_webhook_config.invoice_id.clone(),
                 context.config.test_webhook_config.amount_of_sats,
                 context.config.test_webhook_config.amount_in_fiat,
                 context.config.test_webhook_config.currency.clone(),
                 date.clone(),
                 date.clone())
}

#[derive(Debug)]
pub enum LogEvents {
    TestWebhookStart,
    TestWebhookError,
    TestWebhookNoWebhookConfigured,
    TestWebhookTestOk,
    TestWebhookTestFailed,
    TestWebhookOk,
}

impl fmt::Display for LogEvents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogEvents::TestWebhookStart
                => write!(f, "TestWebhookStart"),
            LogEvents::TestWebhookError
                => write!(f, "TestWebhookError"),
            LogEvents::TestWebhookNoWebhookConfigured
                => write!(f, "TestWebhookNoWebhookConfigured"),
            LogEvents::TestWebhookTestOk
                => write!(f, "TestWebhookTestOk"),
            LogEvents::TestWebhookTestFailed
                => write!(f, "TestWebhookTestFailed"),
            LogEvents::TestWebhookOk
                => write!(f, "TestWebhookOk"),
        }
    }
}
