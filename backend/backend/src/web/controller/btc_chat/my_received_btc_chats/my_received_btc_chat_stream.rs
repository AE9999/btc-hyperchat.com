use actix_web_actors::ws;
use std::time::Instant;
use actix::Addr;
use actix_web::{HttpRequest, HttpResponse, web, Error};
use actix_web::web::Data;
use crate::AppContext;
use crate::web::model::btc_chat::my_received_btc_chat_stream::my_received_btc_chat_streamer::MyReceivedBtcChatStreamer;
use crate::web::model::btc_chat::my_received_btc_chat_stream::my_received_btc_chat_client_session::MyReceivedBtcChatClientSession;


/// Entry point for our websocket route
pub async fn stream_received_btc_chats(req: HttpRequest,
                                       stream: web::Payload,
                                       context: Data<AppContext>,
                                       srv: Data<Addr<MyReceivedBtcChatStreamer>>,
) ->  Result<HttpResponse, Error> {

    let config = context.config.websocket_config.clone();
    let client_session =
        MyReceivedBtcChatClientSession::new(0,
                                            Instant::now(),
                                            srv.get_ref().clone(),
                                            config);

    ws::start(client_session,
                    &req,
                    stream)

}
