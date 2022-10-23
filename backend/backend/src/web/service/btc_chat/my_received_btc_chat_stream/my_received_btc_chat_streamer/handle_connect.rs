use actix::prelude::*;
use crate::web::model::btc_chat::my_received_btc_chat_stream::my_received_btc_chat_streamer::MyReceivedBtcChatStreamer;
use crate::web::model::btc_chat::my_received_btc_chat_stream::messages::connect::Connect;

/// Handler for Connect message.
///
/// Register new session and assign unique id to this session
impl Handler<Connect> for MyReceivedBtcChatStreamer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        log::info!("{:?}, {:?}",
                   LogEvents::HandleConnectStart,
                   msg);

        let id: usize =
            self.register_session(msg.addr);

        log::info!("{:?}",
                   LogEvents::HandleConnectOk);

        id
    }
}

#[derive(Debug)]
enum LogEvents {
    HandleConnectStart,
    HandleConnectOk,
}
