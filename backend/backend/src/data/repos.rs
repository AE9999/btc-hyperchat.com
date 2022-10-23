use crate::config::database::DatabaseConfig;
use crate::data::repository::db_connection::DBConnection;
use crate::data::repository::btc_chat::BtcChatRepository;
use crate::data::repository::store::StoreRepository;
use crate::data::repository::user::UserRepository;

#[derive(Clone)]
pub struct Repositories {
    pub user_repository: UserRepository,
    pub store_repository: StoreRepository,
    pub btc_chat_repository: BtcChatRepository,
}

impl Repositories {

    pub fn new(database_config: &DatabaseConfig) -> Self {
        let db_connection = DBConnection::new(database_config);
        Repositories {
            user_repository: UserRepository::new(&db_connection),
            store_repository: StoreRepository::new(&db_connection),
            btc_chat_repository: BtcChatRepository::new(&db_connection),
        }
    }
}
