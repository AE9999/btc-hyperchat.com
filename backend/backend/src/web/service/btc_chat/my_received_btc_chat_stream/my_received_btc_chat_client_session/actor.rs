use actix::Actor;
use crate::web::model::btc_chat::my_received_btc_chat_stream::my_received_btc_chat_client_session::MyReceivedBtcChatClientSession;
use actix_web_actors::ws;
use actix::prelude::*;
use crate::web::model::btc_chat::my_received_btc_chat_stream::messages::connect::Connect;
use crate::web::model::btc_chat::my_received_btc_chat_stream::messages::disconnect::Disconnect;

impl Actor for MyReceivedBtcChatClientSession {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start.
    /// We register ws session with ChatServer
    fn started(&mut self, ctx: &mut Self::Context) {
        // we'll start heartbeat process on session start.
        self.hb(ctx);

        // register self in chat server. `AsyncContext::wait` register
        // future within context, but context waits until this future resolves
        // before processing any other events.
        // HttpContext::state() is instance of WsChatSessionState, state is shared
        // across all routes within application
        let addr = ctx.address();
        self.addr
            .send(Connect {
                addr: addr.recipient()
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    // something is wrong with chat server
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        // notify chat server
        self.addr.do_send(Disconnect { id: self.id });
        Running::Stop
    }
}
