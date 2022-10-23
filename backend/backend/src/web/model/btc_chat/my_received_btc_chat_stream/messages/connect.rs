use actix::prelude::*;
use crate::web::model::btc_chat::my_received_btc_chat_stream::messages::message::Message;

/// New chat session is created
#[derive(Message, Debug)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<Message>,
}
