use actix::prelude::*;

/// Chat server sends this messages to session
#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct Message(pub String);
