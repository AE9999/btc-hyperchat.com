use btcpay_rust::models::{WebhookDataBaseAuthorizedEvents, WebhookDataCreate};
use crate::{CallbackMapping, AppContext};
use actix_web::http::StatusCode;
use btcpay_rust::apis::webhooks_api::webhooks_create_webhook;
use std::fmt;
use actix_web::web::Data;
use strum::IntoEnumIterator;

pub async fn setup_webhooks_for_btc_pay_store(store_id: &str,
                                          context: &Data<AppContext>) -> Result<(), (StatusCode, String)> {

    log::info!("{:?}, {:?}",
                LogEvents::SetupWebhooksForBtcPayStoreStart,
                store_id);

    let mut webhook_data_create =  WebhookDataCreate::new();
    webhook_data_create.enabled= Some(true);
    webhook_data_create.automatic_redelivery = Some(true);
    webhook_data_create.secret = Some(context.config.btc_pay_config.callback_secret.clone());


    for callback_mapping in CallbackMapping::iter() {
        if !callback_mapping.is_received_payment() {
            // TODO(AE): Redesign we only do processing for now.
            continue;
        }

        let mut webhook_data_create = webhook_data_create.clone();

        let url : String =
            context.config.btc_pay_config.callback_url_base.clone() + callback_mapping.path();

        webhook_data_create.url = Some(url);

        let mut webhook_daata_base_authorized_events =
            WebhookDataBaseAuthorizedEvents::new();

        webhook_daata_base_authorized_events.everything = Some(false);
        webhook_daata_base_authorized_events.specific_events =
            Some(vec!(callback_mapping.event_type().to_string()));

        webhook_data_create.authorized_events =
            Some(Box::new(webhook_daata_base_authorized_events));

        let webhook_data_create =
            webhooks_create_webhook(&context.clients.btc_pay_configuration,
                                    store_id,
                                    webhook_data_create).await;

        if webhook_data_create.is_err() {
            log::error!("{:?}, {:?}, {:?}",
                        LogEvents::SetupWebhooksForBtcPayStoreError,
                        webhook_data_create.as_ref().err().unwrap().to_string(),
                        &context.clients.btc_pay_configuration);
            return Err((StatusCode::INTERNAL_SERVER_ERROR,
                        LogEvents::SetupWebhooksForBtcPayStoreError.to_string()))
        }
    }

    log::info!("{:?}",
                LogEvents::SetupWebhooksForBtcPayStoreOk);

    Ok(())
}

#[derive(Debug)]
enum LogEvents {
    SetupWebhooksForBtcPayStoreStart,
    SetupWebhooksForBtcPayStoreError,
    SetupWebhooksForBtcPayStoreOk,
}

impl fmt::Display for LogEvents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogEvents::SetupWebhooksForBtcPayStoreStart
            => write!(f, "SetupWebhooksForBtcPayStoreStart"),
            LogEvents::SetupWebhooksForBtcPayStoreError
            => write!(f, "SetupWebhooksForBtcPayStoreError"),
            LogEvents::SetupWebhooksForBtcPayStoreOk
            => write!(f, "SetupWebhooksForBtcPayStoreOk"),
        }
    }
}


