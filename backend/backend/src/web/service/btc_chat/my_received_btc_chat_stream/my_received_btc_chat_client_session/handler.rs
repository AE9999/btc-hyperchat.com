use crate::web::model::btc_chat::my_received_btc_chat_stream::my_received_btc_chat_client_session::MyReceivedBtcChatClientSession;
use crate::web::model::btc_chat::my_received_btc_chat_stream::messages::message::Message;
use actix::prelude::*;

/// Handle messages from chat server, we simply send it to peer websocket
impl Handler<Message> for MyReceivedBtcChatClientSession {
    type Result = ();

    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}
