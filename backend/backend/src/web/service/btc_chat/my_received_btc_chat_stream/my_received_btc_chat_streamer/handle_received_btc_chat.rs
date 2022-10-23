use actix::prelude::*;
use crate::web::model::btc_chat::my_received_btc_chat_stream::messages::received_btc_chat::ReceivedBtcChat;
use crate::web::model::btc_chat::my_received_btc_chat_stream::my_received_btc_chat_streamer::MyReceivedBtcChatStreamer;

/// Handler for Connect message.
///
/// Register new session and assign unique id to this session
impl Handler<ReceivedBtcChat> for MyReceivedBtcChatStreamer {
    type Result = ();

    fn handle(&mut self, msg: ReceivedBtcChat, _: &mut Context<Self>) {
        log::info!("{:?}, {:?}",
                   LogEvents::HandleReceivedBtcChatStart,
                   msg);

        self.send_received_btc_chat_to_user(&msg.keycloack_id,
                                  &msg.btc_chat);

        log::info!("{:?}",
                   LogEvents::HandleReceivedBtcChatOk);
    }
}

#[derive(Debug)]
enum LogEvents {
    HandleReceivedBtcChatStart,
    HandleReceivedBtcChatOk,
}
