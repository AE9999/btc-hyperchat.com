use crate::web::model::btc_chat::my_received_btc_chats::my_receiveid_btc_chat::MyReceivedBtcChat;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MyReceivedBtcChatsResponse {
    #[serde(rename = "myReceivedBtcChats")]
    my_received_btc_chats: Vec<MyReceivedBtcChat>,
}

impl MyReceivedBtcChatsResponse {
    pub fn new(my_received_btc_chats: Vec<MyReceivedBtcChat>) -> Self {
        MyReceivedBtcChatsResponse {
            my_received_btc_chats,
        }
    }
}

