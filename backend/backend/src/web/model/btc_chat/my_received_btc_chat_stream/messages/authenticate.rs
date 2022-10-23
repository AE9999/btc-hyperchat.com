use actix::prelude::*;

/// New chat session is created
#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct Authenticate {
    pub id: usize,
    pub web_token: String,
}
