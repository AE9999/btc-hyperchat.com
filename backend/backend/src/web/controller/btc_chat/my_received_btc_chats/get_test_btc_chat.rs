use actix_web::HttpResponse;
use actix_web::http::StatusCode;
use actix_web::web::Data;
use crate::AppContext;
use std::fmt;
use actix_web_middleware_keycloak_auth::StandardKeycloakClaims;
use chrono::Utc;
use uuid::Uuid;
use crate::data::model::btc_chat::{BtcChat, ProcessingStatus};
use crate::web::model::btc_chat::my_received_btc_chats::my_receiveid_btc_chat::MyReceivedBtcChat;
use crate::web::service::btc_chat::my_received_btc_chats::btc_chat_to_my_received_btc_chat::btc_chat_to_my_received_btc_chat;
use crate::web::service::btc_chat::my_received_btc_chats::find_user_by_keycloak_id::find_user_by_keycloak_id;
use crate::web::service::btc_chat::util::find_store_for_user_id::find_store_for_owner_id;


pub async fn get_test_btc_chat(context: Data<AppContext>,
                               claims: StandardKeycloakClaims) -> HttpResponse {

    log::info!("{:?} {:?}",
               LogEvents::GetTestBtcChatStart,
               claims);

    let result =
        do_get_test_btc_chat(&claims.sub,
                             &context).await;

    match result {
        Ok(btc_chat_view) => {
            log::info!("{:?} {:?} ",
                       LogEvents::GetTestBtcChatOk,
                       btc_chat_view);
            HttpResponse::Ok().json(btc_chat_view)
        },
        Err(error) => {
            log::error!("{:?} {:?} ",
                        LogEvents::GetTestBtcChatError,
                        error.1.clone());
            HttpResponse::build(error.0.clone())
                .body(LogEvents::GetTestBtcChatError.to_string())
        }
    }
}


async fn do_get_test_btc_chat(keycloak_id: &Uuid,
                              context: &Data<AppContext>) -> Result<MyReceivedBtcChat,
                                                                    (StatusCode, String)> {
    // TODO(Ae): write a single query for optimization

    let user =
        find_user_by_keycloak_id(keycloak_id,
                                 context)?;

    let store =
        find_store_for_owner_id(&user.id,
                                context)?;

    let now  = Utc::now();

    let btc_chat =
        BtcChat::new(context.config.test_webhook_config.id.clone(),
                 true,
                 ProcessingStatus::Confirmed.to_code(),
                 Some(context.config.test_webhook_config.message.clone()),
                 Some(context.config.test_webhook_config.sender.clone()),
                 store.btcpay_store_id,
                 context.config.test_webhook_config.invoice_id.clone(),
                 context.config.test_webhook_config.amount_of_sats,
                 context.config.test_webhook_config.amount_in_fiat,
                 context.config.test_webhook_config.currency.clone(),
                 now.clone(),
                 now);

    let my_received_btc_chat =
        btc_chat_to_my_received_btc_chat(btc_chat);

    Ok(my_received_btc_chat)
}

#[derive(Debug)]
pub enum LogEvents {
    GetTestBtcChatStart,
    GetTestBtcChatOk,
    GetTestBtcChatError,
}

impl fmt::Display for LogEvents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogEvents::GetTestBtcChatStart => write!(f, "GetTestBtcChatStart"),
            LogEvents::GetTestBtcChatOk => write!(f, "GetTestBtcChatOk"),
            LogEvents::GetTestBtcChatError => write!(f, "GetTestBtcChatError"),

        }
    }
}
