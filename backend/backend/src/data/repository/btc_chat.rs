use uuid::Uuid;
use chrono::Utc;
use diesel::prelude::*;
use simple_error::SimpleError;
use crate::data::repository::db_connection::DBConnection;
use crate::data::model::btc_chat::{BtcChat, ProcessingStatusValue};
use crate::data::schema;
use crate::data::repository::db_event::DbEvent;
use crate::data::schema::btc_chats::dsl::btc_chats;
use crate::data::schema::btc_chats::{date_created, id};


#[derive(Clone, Debug)]
pub struct BtcChatRepository {
    db_connection: DBConnection
}

impl BtcChatRepository {
    pub fn new(db_connection: &DBConnection) -> Self {
        BtcChatRepository {
            db_connection: db_connection.clone()
        }
    }

    #[allow(dead_code)]
    pub fn find_by_pk(&self, id_: &Uuid) ->  Result<Option<BtcChat>,
                                                    SimpleError> {
        let connection =
            self.db_connection.establish_connection()?;

        let result: QueryResult<Vec<BtcChat>> =
            btc_chats.find(id_)
                     .load::<BtcChat>(&connection);

        if result.is_err() {
            log::error!("{:?} {:?}",
                       DbEvent::DbQueryFailure,
                       result.as_ref().err().unwrap());
            return Err(SimpleError::from(result.err().unwrap()));
        }

        let mut result = result.unwrap();

        Ok(if result.is_empty() { None }
           else { Some(result.remove(0)) })
    }

    pub fn find_by_store_id_and_processing_status_code_order_by_date_created_asc(&self,
                                                                                 store_id_: &str,
                                                                                 processing_status_code_: &i32)
        -> Result<Vec<BtcChat>, SimpleError> {

        let connection =
            self.db_connection.establish_connection()?;

        let result: QueryResult<Vec<BtcChat>> =
            btc_chats.filter(schema::btc_chats::columns::store_id
                                        .eq(store_id_)
                                            .and(schema::btc_chats::columns::processing_status_code
                                                .eq(processing_status_code_)))
                     .order_by(date_created.asc())
                .load::<BtcChat>(&connection);

        if result.is_err() {
            log::error!("{:?} {:?}",
                       DbEvent::DbQueryFailure,
                       result.as_ref().err().unwrap());
            return Err(SimpleError::from(result.err().unwrap()));
        }

        let result = result.unwrap();

        Ok(result)
    }
    
    pub fn find_by_invoice_id(&self, invoice_id_: &str) -> Result<Option<BtcChat>,
                                                                     SimpleError> {

        let connection =
            self.db_connection.establish_connection()?;

        let result: QueryResult<Vec<BtcChat>> =
            btc_chats.filter(schema::btc_chats::columns::invoice_id.eq(invoice_id_))
                     .load::<BtcChat>(&connection);

        if result.is_err() {
            log::error!("{:?} {:?}",
                       DbEvent::DbQueryFailure,
                       result.as_ref().err().unwrap());
            return Err(SimpleError::from(result.err().unwrap()));
        }

        let mut result = result.unwrap();

        Ok(if result.is_empty() { None }
        else { Some(result.remove(0)) })
    }

    pub fn create(&self,
                  active_: bool,
                  processing_status_code_: ProcessingStatusValue,
                  message_: Option<String>,
                  sender_: Option<String>,
                  store_id_: String,
                  invoice_id_: String,
                  amount_of_sats_: i64,
                  amount_in_fiat_: i32,
                  currency_: String,
    ) -> Result<BtcChat, SimpleError> {

        let now  = Utc::now();
        let btc_chat = BtcChat::new(Uuid::new_v4(),
                                   active_,
                                    processing_status_code_,
                                    message_,
                                    sender_,
                                 store_id_,
                                invoice_id_,
                                        amount_of_sats_,
                                        amount_in_fiat_,
                                        currency_,
                                    now,
                                    now);

        let connection =
            self.db_connection.establish_connection()?;

        let statement =
            diesel::insert_into(schema::btc_chats::table)
                   .values(&btc_chat)
                   .execute(&connection);

        if statement.is_err() {
            log::error!("{:?} {:?}",
                       DbEvent::DbQueryFailure,
                       statement.as_ref().err().unwrap());
            return Err(SimpleError::from(statement.err().unwrap()));
        }

        log::info!("{:?} {:?}", DbEvent::DbEntityCreation, btc_chat);

        Ok(btc_chat)
    }


    // update
    pub fn update(&self, btc_chat: &BtcChat) -> Result<(), SimpleError> {

        let connection =
            self.db_connection.establish_connection()?;

        let filter = id.eq(btc_chat.id);

        let result =
            diesel::update(schema::btc_chats::table.filter(filter))
                .set(btc_chat).execute(&connection);

        if result.is_err() {
            log::error!("{:?} {:?}",
                       DbEvent::DbQueryFailure,
                       result.as_ref().err().unwrap());
            return Err(SimpleError::from(result.err().unwrap()));
        }

        log::info!("{:?} {:?}",
                   DbEvent::DbEntityUpdate,
                   btc_chats);

        Ok(())
    }
}
