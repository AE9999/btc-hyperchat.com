use std::time::{Duration, Instant};

use actix::prelude::*;
use actix_web_actors::ws;
use crate::config::websocket::WebsocketConfig;
use crate::web::model::btc_chat::my_received_btc_chat_stream::my_received_btc_chat_streamer::MyReceivedBtcChatStreamer;
use crate::web::model::btc_chat::my_received_btc_chat_stream::messages::disconnect::Disconnect;

#[derive(Debug)]
pub struct MyReceivedBtcChatClientSession {
    /// unique session id
    pub id: usize,

    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    pub hb: Instant,

    /// Chat server
    pub addr: Addr<MyReceivedBtcChatStreamer>,

    pub web_socket_config: WebsocketConfig,
}

impl MyReceivedBtcChatClientSession {
    pub fn new(id : usize,
               hb: Instant,
               addr: Addr<MyReceivedBtcChatStreamer>,
               web_socket_config: WebsocketConfig) -> Self {
        MyReceivedBtcChatClientSession {
            id,
            hb,
            addr,
            web_socket_config,
        }
    }

    /// helper method that sends ping to client every 5 seconds (HEARTBEAT_INTERVAL).
    ///
    /// also this method checks heartbeats from client
    pub fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        let interval = Duration::from_secs(self.web_socket_config.heartbeat_interval.clone());
        let client_timeout = self.web_socket_config.client_timeout.clone();

        ctx.run_interval(interval, move |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) >
                Duration::from_secs(client_timeout) {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");

                // notify chat server
                act.addr.do_send(Disconnect { id: act.id });

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }
}
