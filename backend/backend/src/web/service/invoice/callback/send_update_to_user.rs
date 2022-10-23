use std::fmt;
use actix::Addr;
use actix_web::http::StatusCode;
use actix_web::web::Data;
use uuid::Uuid;
use crate::{AppContext, MyReceivedBtcChatStreamer};
use crate::data::model::btc_chat::BtcChat;
use crate::web::model::btc_chat::my_received_btc_chat_stream::messages::received_btc_chat::ReceivedBtcChat;
use crate::web::service::btc_chat::process_btc_chat::set_chat_status_as_processed_in_db::set_chat_status_as_processed_in_db;
use crate::web::service::invoice::callback::find_owner_for_store::find_owner_for_store;
use crate::web::service::invoice::callback::find_store_for_btc_chat::find_store_for_btc_chat;
use crate::web::service::invoice::callback::webhook::make_callback_per_store_webhook_config::make_callback_per_store_webhook_config;
use crate::web::service::profile::deserialize_store_webhook_config_json::deserialize_store_webhook_config_json;

pub async fn send_update_to_user(btc_chat: &mut BtcChat,
                                 context: &Data<AppContext>,
                                 my_received_btc_chat_streamer: &Data<Addr<MyReceivedBtcChatStreamer>>)
        -> Result<(), (StatusCode, String)> {

    log::info!("{:?}",
               LogEvents::SendUpdateToUserStart);

    let store = find_store_for_btc_chat(btc_chat,
                                              context)?;
    let user =
                    find_owner_for_store(&store,
                                         context)?;

    let mut processed = false;

    if store.webhook_active {

        let store_webhook_config_json =
            store.webhook_config_json.as_str();

        let store_webhook_config =
            deserialize_store_webhook_config_json(store_webhook_config_json)?;

        let ok =
            make_callback_per_store_webhook_config(&store_webhook_config,
                                                   btc_chat,
                                                   context).await;

        if !ok.is_err()
            && store.automatically_process_btc_chats_if_webhook_succeeds {
            let ok =
                set_chat_status_as_processed_in_db(btc_chat,
                                                    context);

            processed = !ok.is_err();
        }
    }

    if !processed {
        send_callback_to_user_interface(&user.keycloak_id,
                                        btc_chat,
                                        my_received_btc_chat_streamer).await?;
    }

    log::info!("{:?}",
               LogEvents::SendUpdateToUserOk);

    Ok(())
}

async fn send_callback_to_user_interface(keycloak_id: &Uuid,
                                         btc_chat: &mut BtcChat,
                                         my_received_btc_chat_streamer: &Data<Addr<MyReceivedBtcChatStreamer>>) -> Result<(),
                                        (StatusCode, String)> {
    log::info!("{:?}, {:?}, {:?}",
               LogEvents::SendUpdateToGuiStart,
               keycloak_id,
               btc_chat);

    my_received_btc_chat_streamer.do_send(ReceivedBtcChat { keycloack_id: keycloak_id.clone(),
                                                                 btc_chat:btc_chat.clone() });
    log::info!("{:?}",
               LogEvents::SendUpdateToGuiOk);
    Ok(())
}

#[derive(Debug)]
enum LogEvents {
    SendUpdateToUserStart,
    SendUpdateToUserOk,

    SendUpdateToGuiStart,
    SendUpdateToGuiOk,
}

impl fmt::Display for LogEvents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogEvents::SendUpdateToUserStart
                => write!(f, "SendUpdateToUserStart"),
            LogEvents::SendUpdateToUserOk
                => write!(f, "SendUpdateToUserOk"),
            LogEvents::SendUpdateToGuiStart
                => write!(f, "SendUpdateToGuiStart"),
            LogEvents::SendUpdateToGuiOk
                => write!(f, "SendUpdateToGuiOk"),
        }
    }
}
