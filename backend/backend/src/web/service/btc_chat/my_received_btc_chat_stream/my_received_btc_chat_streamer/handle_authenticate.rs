use actix::prelude::*;
use crate::web::model::btc_chat::my_received_btc_chat_stream::messages::authenticate::Authenticate;
use crate::web::model::btc_chat::my_received_btc_chat_stream::my_received_btc_chat_streamer::MyReceivedBtcChatStreamer;

/// Handler for Connect message.
///
/// Register new session and assign unique id to this session
impl Handler<Authenticate> for MyReceivedBtcChatStreamer {
    type Result = ();

    fn handle(&mut self, msg: Authenticate, _: &mut Context<Self>) {
        log::info!("{:?}, {:?}",
                   LogEvents::HandleAuthenticateStart,
                   msg);

        self.authenticate_session(msg.id,
                                  msg.web_token.as_str());

        log::info!("{:?}",
                   LogEvents::HandleAuthenticateOk);
    }
}

#[derive(Debug)]
enum LogEvents {
    HandleAuthenticateStart,
    HandleAuthenticateOk,
}
