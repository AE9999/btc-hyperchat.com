use std::fmt;
use uuid::Uuid;
use crate::data::model::btc_chat::BtcChat;
use crate::web::service::btc_chat::util::find_store_for_user_id::find_store_for_owner_id;
use crate::AppContext;
use actix_web::http::StatusCode;
use actix_web::web::Data;
use crate::web::service::btc_chat::my_received_btc_chats::find_user_by_keycloak_id::find_user_by_keycloak_id;

pub fn verify_poster_is_owner_of_chat(keycloak_id: &Uuid,
                                      btc_chat: &BtcChat,
                                      context: &Data<AppContext>) -> Result<(), (StatusCode, String)> {

    log::info!("{:?}, {:?}, {:?}",
                LogEvents::VerifyPosterIsOwnerOfChatStart,
                keycloak_id,
                btc_chat);

    let user =
        find_user_by_keycloak_id(keycloak_id,
                                 context)?;

    let store =
        find_store_for_owner_id(&user.id,
                                context)?;

    // TOOD(AE):Check these ids are correct.
    if btc_chat.store_id != store.btcpay_store_id {

        log::info!("{:?}",
                   LogEvents::VerifyPosterIsOwnerOfChatError);

        return Err((StatusCode::BAD_REQUEST,
                    LogEvents::VerifyPosterIsOwnerOfChatError.to_string()))
    }

    log::info!("{:?}",
                LogEvents::VerifyPosterIsOwnerOfChatOk);

    Ok(())
}

#[derive(Debug)]
pub enum LogEvents {
    VerifyPosterIsOwnerOfChatStart,
    VerifyPosterIsOwnerOfChatOk,
    VerifyPosterIsOwnerOfChatError,
}

impl fmt::Display for LogEvents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogEvents::VerifyPosterIsOwnerOfChatStart => write!(f, "VerifyPosterIsOwnerOfChatStart"),
            LogEvents::VerifyPosterIsOwnerOfChatOk => write!(f, "VerifyPosterIsOwnerOfChatOk"),
            LogEvents::VerifyPosterIsOwnerOfChatError => write!(f, "VerifyPosterIsOwnerOfChatError"),
        }
    }
}
