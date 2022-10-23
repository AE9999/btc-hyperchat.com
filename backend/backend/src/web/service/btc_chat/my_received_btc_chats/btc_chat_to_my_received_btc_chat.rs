use crate::data::model::btc_chat::BtcChat;
use crate::web::model::btc_chat::my_received_btc_chats::my_receiveid_btc_chat::MyReceivedBtcChat;


pub fn btc_chat_to_my_received_btc_chat(btc_chat: BtcChat) -> MyReceivedBtcChat {
    MyReceivedBtcChat::new(btc_chat.id.to_string(),
                           btc_chat.message,
                           btc_chat.sender,
                           btc_chat.amount_of_sats,
                           btc_chat.amount_in_fiat,
                           btc_chat.currency,
                           btc_chat.date_created.to_string())
}
