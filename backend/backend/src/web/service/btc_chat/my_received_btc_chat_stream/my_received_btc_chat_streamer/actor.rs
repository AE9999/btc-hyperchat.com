use actix::{Actor, Context};
use crate::web::model::btc_chat::my_received_btc_chat_stream::my_received_btc_chat_streamer::MyReceivedBtcChatStreamer;

/// Make actor from `ChatServer`
impl Actor for MyReceivedBtcChatStreamer {
    /// We are going to use simple Context, we just need ability to communicate
    /// with other actors.
    type Context = Context<Self>;
}
