use actix::prelude::*;
use crate::web::model::btc_chat::my_received_btc_chat_stream::my_received_btc_chat_streamer::MyReceivedBtcChatStreamer;
use crate::web::model::btc_chat::my_received_btc_chat_stream::messages::disconnect::Disconnect;

/// Handler for Disconnect message.
impl Handler<Disconnect> for MyReceivedBtcChatStreamer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        log::info!("{:?} {:?} ",
                   LogEvents::HandleDisconnectStart,
                   msg);

        self.destroy_session(msg.id);

        log::info!("{:?}",
                   LogEvents::HandleDisconnectOk);
    }
}

#[derive(Debug)]
enum LogEvents {
    HandleDisconnectStart,
    HandleDisconnectOk,
}
