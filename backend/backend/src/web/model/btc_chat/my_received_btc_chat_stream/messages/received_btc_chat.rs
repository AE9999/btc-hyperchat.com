use actix::prelude::*;
use uuid::Uuid;
use crate::data::model::btc_chat::BtcChat;

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct ReceivedBtcChat {
    pub keycloack_id: Uuid,
    pub btc_chat: BtcChat,
}
