use std::collections::HashMap;
use rand::{self, Rng, rngs::ThreadRng};
use crate::data::model::btc_chat::BtcChat;
use crate::web::model::btc_chat::my_received_btc_chat_stream::messages::message::Message;
use std::fmt;
use std::sync::Mutex;
use actix_web_middleware_keycloak_auth::{DecodingKey, StandardClaims };
use jsonwebtoken::{Algorithm, decode, Validation};
use uuid::Uuid;
use crate::config::keycloak::KeycloakConfig;
use crate::get_decoding_key;
use crate::web::service::btc_chat::my_received_btc_chats::btc_chat_to_my_received_btc_chat::btc_chat_to_my_received_btc_chat;

/*#[derive(Clone)]*/
pub struct MyReceivedBtcChatStreamer {
    sessions: HashMap<usize, actix::Recipient<Message>>,
    keycloak_id_2_session: HashMap<Uuid, usize>,
    session_id_2_keycloak: HashMap<usize, Uuid>,
    rng: ThreadRng,
    keycloak_pk: DecodingKey,
    mutex: Mutex<bool>,
}

impl MyReceivedBtcChatStreamer {

    pub fn new(keycloak_config: &KeycloakConfig) -> Self {
        log::info!("{:?}",
                   LogEvents::MyReceivedBtcChatStreamerNewStart);

        // default room
        let my_received_btc_chat_streamer
            = MyReceivedBtcChatStreamer {

            sessions: HashMap::new(),
            keycloak_id_2_session: HashMap::new(),
            session_id_2_keycloak: HashMap::new(),
            rng: rand::thread_rng(),
            keycloak_pk : get_decoding_key(keycloak_config),
            mutex: Mutex::new(false),
        };

        log::info!("{:?}",
                   LogEvents::MyReceivedBtcChatStreamerNewOk);

        my_received_btc_chat_streamer
    }

    pub fn register_session(&mut self,
                            addr: actix::Recipient<Message>) -> usize {
        let _lock = self.mutex.lock();
        log::info!("{:?}",
                   LogEvents::MyReceivedBtcChatStreamerRegisterSessionStart);

        let mut session_id = self.rng.gen::<usize>();
        loop {
            if !self.sessions.contains_key(&session_id) {
                break;
            }
            session_id = self.rng.gen::<usize>();
        }
        self.sessions.insert(session_id, addr);

        log::info!("{:?}",
                   LogEvents::MyReceivedBtcChatStreamerRegisterSessionOk);

        session_id
    }

    pub fn authenticate_session(&mut self,
                                session_id: usize,
                                json_web_token: &str) {
        let _lock = self.mutex.lock();
        log::info!("{:?} {:?}",
                   LogEvents::MyReceivedBtcChatStreamerAuthenticationStart,
                   json_web_token);

        let token_data =
            decode::<StandardClaims>(json_web_token,
                         &self.keycloak_pk,
                     &Validation::new(Algorithm::RS256)); // Make this configurable

        if token_data.is_err() {
            log::info!("{:?}",
                   LogEvents::MyReceivedBtcChatStreamerAuthenticationInvalidToken);
            return
        }

        let token_data = token_data.unwrap();

        let keycloak_id = token_data.claims.sub;

        self.keycloak_id_2_session.insert(keycloak_id.clone(), session_id);
        self.session_id_2_keycloak.insert(session_id, keycloak_id.clone());

        log::info!("{:?}",
                   LogEvents::MyReceivedBtcChatStreamerAuthenticationOk);
    }

    pub fn destroy_session(&mut self,
                            session_id: usize) {

        log::info!("{:?}, {:?}",
                   LogEvents::MyReceivedBtcChatStreamerDestroySessionStart,
                   session_id);
        let _lock = self.mutex.lock();
        self.sessions.remove(&session_id);

        if self.session_id_2_keycloak.contains_key(&session_id) {
            /* Session may not have authenticated */
            let keycloack_id =
                self.session_id_2_keycloak
                    .get(&session_id)
                    .expect(LogEvents::MyReceivedBtcChatStreamerDestroySessionAssertionError.to_string().as_str())
                    .clone();

            self.session_id_2_keycloak.remove(&session_id);

            self.keycloak_id_2_session.remove(&keycloack_id);
        }

        log::info!("{:?}",
                   LogEvents::MyReceivedBtcChatStreamerDestroySessionOk);
    }

    pub fn send_received_btc_chat_to_user(&self,
                                          keycloak_id: &Uuid,
                                          btc_chat: &BtcChat) {
        log::info!("{:?}, {:?} {:?}",
                   LogEvents::MyReceivedBtcChatStreamerSendReceivedBtcChatToUserStart,
                   keycloak_id,
                   btc_chat);

        let _lock = self.mutex.lock();

        let session =
            self.keycloak_id_2_session
                .get(keycloak_id);

        if session.is_none() {

            log::info!("{:?}",
                   LogEvents::MyReceivedBtcChatStreamerSendReceivedBtcChatToUserUserNotConnected);

            log::info!("{:?}",
                   LogEvents::MyReceivedBtcChatStreamerSendReceivedBtcChatToUserUserOk);

            return;
        }

        let session = session.unwrap();

        log::info!("{:?}",
                   LogEvents::MyReceivedBtcChatStreamerSendReceivedBtcChatToUserUserConnectedAndSendingNotification);

        let addr =
            self.sessions
               .get(session)
               .expect(LogEvents::MyReceivedBtcChatStreamerSendReceivedBtcChatAssertionError.to_string().as_str());

        let my_received_btc_chat =
            btc_chat_to_my_received_btc_chat(btc_chat.clone());

        let message = serde_json::to_string(&my_received_btc_chat);

        if message.is_err() {
            log::error!("{:?} {:?}",
                        LogEvents::MyReceivedBtcChatStreamerSendReceivedBtcChatSerializationError,
                        message.err().unwrap().to_string());
            return;
        }

        let message= message.unwrap();

        addr.do_send(Message(message));

        log::info!("{:?}",
                   LogEvents::MyReceivedBtcChatStreamerSendReceivedBtcChatToUserUserOk);

    }
}

#[derive(Debug)]
enum LogEvents {
    MyReceivedBtcChatStreamerNewStart,
    MyReceivedBtcChatStreamerNewOk,

    MyReceivedBtcChatStreamerAuthenticationStart,
    MyReceivedBtcChatStreamerAuthenticationInvalidToken,
    MyReceivedBtcChatStreamerAuthenticationOk,

    MyReceivedBtcChatStreamerRegisterSessionStart,
    MyReceivedBtcChatStreamerRegisterSessionOk,

    MyReceivedBtcChatStreamerDestroySessionStart,
    MyReceivedBtcChatStreamerDestroySessionAssertionError,
    MyReceivedBtcChatStreamerDestroySessionOk,

    MyReceivedBtcChatStreamerSendReceivedBtcChatToUserStart,
    MyReceivedBtcChatStreamerSendReceivedBtcChatAssertionError,
    MyReceivedBtcChatStreamerSendReceivedBtcChatSerializationError,
    MyReceivedBtcChatStreamerSendReceivedBtcChatToUserUserNotConnected,
    MyReceivedBtcChatStreamerSendReceivedBtcChatToUserUserConnectedAndSendingNotification,
    MyReceivedBtcChatStreamerSendReceivedBtcChatToUserUserOk,
}

impl fmt::Display for LogEvents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogEvents::MyReceivedBtcChatStreamerNewStart
                => write!(f, "MyReceivedBtcChatStreamerNewStart"),
            LogEvents::MyReceivedBtcChatStreamerNewOk
                => write!(f, "MyReceivedBtcChatStreamerNewOk"),

            LogEvents::MyReceivedBtcChatStreamerAuthenticationStart
                => write!(f, "MyReceivedBtcChatStreamerAuthenticationStart"),
            LogEvents::MyReceivedBtcChatStreamerAuthenticationInvalidToken
                => write!(f, "MyReceivedBtcChatStreamerAuthenticationInvalidToken"),
            LogEvents::MyReceivedBtcChatStreamerAuthenticationOk
                => write!(f, "MyReceivedBtcChatStreamerAuthenticationOk"),

            LogEvents::MyReceivedBtcChatStreamerRegisterSessionStart
                => write!(f, "MyReceivedBtcChatStreamerRegisterSessionStart"),
            LogEvents::MyReceivedBtcChatStreamerRegisterSessionOk
                => write!(f, "MyReceivedBtcChatStreamerRegisterSessionOk"),
            LogEvents::MyReceivedBtcChatStreamerDestroySessionStart
                => write!(f, "MyReceivedBtcChatStreamerDestroySessionStart"),
            LogEvents::MyReceivedBtcChatStreamerSendReceivedBtcChatSerializationError
                => write!(f, "MyReceivedBtcChatStreamerSendReceivedBtcChatSerializationError"),
            LogEvents::MyReceivedBtcChatStreamerDestroySessionAssertionError
                => write!(f, "MyReceivedBtcChatStreamerDestroySessionAssertionError"),
            LogEvents::MyReceivedBtcChatStreamerDestroySessionOk
                => write!(f, "MyReceivedBtcChatStreamerDestroySessionOk"),
            LogEvents::MyReceivedBtcChatStreamerSendReceivedBtcChatToUserStart
                => write!(f, "MyReceivedBtcChatStreamerSendReceivedBtcChatToUserStart"),
            LogEvents::MyReceivedBtcChatStreamerSendReceivedBtcChatToUserUserNotConnected
                => write!(f, "MyReceivedBtcChatStreamerSendReceivedBtcChatToUserUserNotConnected"),
            LogEvents::MyReceivedBtcChatStreamerSendReceivedBtcChatToUserUserConnectedAndSendingNotification
                => write!(f, "MyReceivedBtcChatStreamerSendReceivedBtcChatToUserUserConnectedAndSendingNotification"),
            LogEvents::MyReceivedBtcChatStreamerSendReceivedBtcChatToUserUserOk
                => write!(f, "MyReceivedBtcChatStreamerSendReceivedBtcChatToUserUserOk"),
            LogEvents::MyReceivedBtcChatStreamerSendReceivedBtcChatAssertionError
                => write!(f, "MyReceivedBtcChatStreamerSendReceivedBtcChatAssertionError"),


        }
    }
}


