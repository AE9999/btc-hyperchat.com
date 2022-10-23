use crate::web::model::btc_chat::my_received_btc_chat_stream::my_received_btc_chat_client_session::MyReceivedBtcChatClientSession;
use actix::prelude::*;
use actix_web_actors::ws;
use std::time::Instant;
use std::fmt;
use crate::web::model::btc_chat::my_received_btc_chat_stream::messages::authenticate::Authenticate;

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyReceivedBtcChatClientSession {

    fn handle(&mut self,
              msg: Result<ws::Message, ws::ProtocolError>,
              ctx: &mut Self::Context) {

        let msg = match msg {
            Err(protocol_error) => {

                log::debug!("{:?} {:?}",
                            LogEvents::ClientWebSocketProtocolError,
                            protocol_error);

                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        log::debug!("{:?} {:?}",
                   LogEvents::ClientWebSocketMessageReceived,
                   msg);

        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Text(text) => {
                // we only use it for communication
                //let m = text.trim();
                let parsed_message: serde_json::Result<ClientMessage> =
                    serde_json::from_str(text.trim());

                if parsed_message.is_err() {
                    log::error!("{:?}. {:?}",
                               LogEvents::ClientWebSocketUnparsableClientMessage,
                               parsed_message.as_ref().err().unwrap());
                    ctx.text("!!! ".to_owned() +
                             LogEvents::ClientWebSocketUnparsableClientMessage.to_string().as_str());
                    return;
                }

                let parsed_message = parsed_message.unwrap();

                // we check for /sss type of messages
                if parsed_message.message.starts_with('/') {
                    log::info!("{:?}",
                               LogEvents::ClientWebSocketExecutingCommand);
                    let v: Vec<&str> = parsed_message.message.splitn(2, ' ').collect();
                    match v[0] {
                        "/authenticate" => {
                            if v.len() == 2 {
                                log::info!("{:?}",
                                            LogEvents::ClientWebSocketExecutingAuthenticateCommand);

                                self.addr.do_send(Authenticate {
                                    id: self.id,
                                    web_token: String::from(v[1]),
                                });
                            } else {
                                log::info!("{:?}",
                                            LogEvents::ClientWebSocketFailedAuthenticateCommand);

                                ctx.text("!!! Web token is required");
                            }
                        },
                        _ => ctx.text(format!("!!! unknown command:")),
                    }
                }
            }
            ws::Message::Binary(_) => {
                // We only use it for one way
            },
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            ws::Message::Continuation(_) => {
                ctx.stop();
            }
            ws::Message::Nop => (),
        }

        log::debug!("{:?}",
                   LogEvents::ClientWebSocketMessageProcessed);
    }
}


#[derive(Clone, Debug)]
enum LogEvents {
    ClientWebSocketProtocolError,
    ClientWebSocketExecutingAuthenticateCommand,
    ClientWebSocketUnparsableClientMessage,
    ClientWebSocketFailedAuthenticateCommand,
    ClientWebSocketMessageReceived,
    ClientWebSocketMessageProcessed,
    ClientWebSocketExecutingCommand,
}


// TODO(AE): Put this in a correct package
#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct ClientMessage {
    pub message: String,
}

impl fmt::Display for LogEvents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogEvents::ClientWebSocketProtocolError
                => write!(f, "ClientWebSocketProtocolError"),
            LogEvents::ClientWebSocketExecutingAuthenticateCommand
                => write!(f, "ClientWebSocketExecutingAuthenticateCommand"),
            LogEvents::ClientWebSocketUnparsableClientMessage
                => write!(f, "ClientWebSocketUnparsableClientMessage"),
            LogEvents::ClientWebSocketFailedAuthenticateCommand
                => write!(f, "ClientWebSocketFailedAuthenticateCommand"),
            LogEvents::ClientWebSocketMessageReceived
                => write!(f, "ClientWebSocketMessageReceived"),
            LogEvents::ClientWebSocketMessageProcessed
                => write!(f, "ClientWebSocketMessageProcessed"),
            LogEvents::ClientWebSocketExecutingCommand
                => write!(f, "ClientWebSocketExecutingCommand"),
        }
    }
}

