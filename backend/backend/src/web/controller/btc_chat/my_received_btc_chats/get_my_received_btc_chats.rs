use actix_web::HttpResponse;
use actix_web::http::StatusCode;
use actix_web::web::Data;
use crate::AppContext;
use actix_web_middleware_keycloak_auth::StandardKeycloakClaims;
use uuid::Uuid;
use crate::web::model::btc_chat::my_received_btc_chats::my_received_btc_chats_response::MyReceivedBtcChatsResponse;
use crate::web::service::btc_chat::util::find_store_for_user_id::find_store_for_owner_id;
use crate::web::service::btc_chat::my_received_btc_chats::find_confirmed_chats_for_store_id::find_new_btc_chats_for_store_id;
use crate::web::service::btc_chat::my_received_btc_chats::find_user_by_keycloak_id::find_user_by_keycloak_id;
use std::fmt;
use crate::web::service::btc_chat::my_received_btc_chats::btc_chat_to_my_received_btc_chat::btc_chat_to_my_received_btc_chat;


pub async fn get_my_received_btc_chats(context: Data<AppContext>,
                                       claims: StandardKeycloakClaims) -> HttpResponse {

    log::info!("{:?} {:?}",
               LogEvents::GetCurrentBtcChatsStart,
               claims);

    let result =
        do_get_my_received_btc_chats(&claims.sub,
                                     &context).await;

    match result {
        Ok(btc_chat_view) => {
            log::info!("{:?} {:?} ",
                       LogEvents::GetCurrentBtcChatsOk,
                       btc_chat_view);
            HttpResponse::Ok().json(btc_chat_view)
        },
        Err(error) => {
            log::error!("{:?} {:?} ",
                        LogEvents::GetCurrentBtcChatsError,
                        error.1.clone());
            HttpResponse::build(error.0.clone())
                .body(LogEvents::GetCurrentBtcChatsError.to_string())
        }
    }
}


async fn do_get_my_received_btc_chats(keycloak_id: &Uuid,
                                      context: &Data<AppContext>) -> Result<MyReceivedBtcChatsResponse,
                                                                      (StatusCode, String)> {
    // TODO(Ae): write a single query for optimization

    let user =
        find_user_by_keycloak_id(keycloak_id,
                                 context)?;

    let store =
        find_store_for_owner_id(&user.id,
                                context)?;

    let btc_chats =
        find_new_btc_chats_for_store_id(&store.btcpay_store_id,
                                        context)?;

    let btc_chats =
        btc_chats.into_iter()
                 .map(|btc_chat| btc_chat_to_my_received_btc_chat(btc_chat))
                 .collect();

    let my_received_btc_chat_response =
        MyReceivedBtcChatsResponse::new(btc_chats);

    Ok(my_received_btc_chat_response)
}

#[derive(Debug)]
pub enum LogEvents {
    GetCurrentBtcChatsStart,
    GetCurrentBtcChatsOk,
    GetCurrentBtcChatsError,
}

impl fmt::Display for LogEvents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogEvents::GetCurrentBtcChatsStart => write!(f, "GetCurrentBtcChatsStart"),
            LogEvents::GetCurrentBtcChatsOk => write!(f, "GetCurrentBtcChatsOk"),
            LogEvents::GetCurrentBtcChatsError => write!(f, "GetCurrentBtcChatsError"),

        }
    }
}



