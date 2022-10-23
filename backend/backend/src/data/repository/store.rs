use simple_error::SimpleError;
use uuid::Uuid;
use chrono::Utc;
use diesel::prelude::*;
use crate::data::repository::db_connection::DBConnection;
use crate::data::model::store::Store;
use crate::data::schema::stores::dsl::*;
use crate::data::schema;
use crate::data::repository::db_event::DbEvent;

#[derive(Clone, Debug)]
pub struct StoreRepository {
    db_connection: DBConnection
}

impl StoreRepository {
    pub fn new(db_connection: &DBConnection) -> Self {
        StoreRepository {
            db_connection: db_connection.clone()
        }
    }

    #[allow(dead_code)]
    pub fn find_by_pk(&self, id_: &Uuid) ->  Result<Option<Store>, SimpleError> {

        let connection =
            self.db_connection.establish_connection()?;

        let result: QueryResult<Vec<Store>> =
            stores.find(id_)
                  .load::<Store>(&connection);

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

    pub fn find_by_btcpay_store_id(&self, btcpay_store_id_: &str) ->
                                                                     Result<Option<Store>, SimpleError> {

        let connection =
            self.db_connection.establish_connection()?;

        let result: QueryResult<Vec<Store>> =
            stores.filter(schema::stores::columns::btcpay_store_id.eq(btcpay_store_id_))
                  .load::<Store>(&connection);

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

    pub fn find_by_owner_id(&self, owner_id_: &Uuid) -> Result<Vec<Store>, SimpleError> {

        let connection =
            self.db_connection.establish_connection()?;

        let result: QueryResult<Vec<Store>> =
            stores.filter(schema::stores::columns::owner_id.eq(owner_id_))
                  .load::<Store>(&connection);

        let result = result.unwrap();

        Ok(result)
    }

    pub fn create(&self,
                  owner_id_: Uuid,
                  btcpay_store_id_: String,
                  webhook_config_json_: String,
                  webhook_active_: bool,
                  automatically_process_btc_chats_if_webhook_succeeds_: bool,
                  active_: bool) -> Result<Store, SimpleError> {

        let now  = Utc::now();

        let store = Store::new(Uuid::new_v4(),
                              owner_id_,
                              btcpay_store_id_,
                               webhook_config_json_,
                               webhook_active_,
                               automatically_process_btc_chats_if_webhook_succeeds_,
                               active_,
                             now,
                             now);

        let connection =
            self.db_connection.establish_connection()?;

        let statement =
            diesel::insert_into(schema::stores::table)
                   .values(&store)
                   .execute(&connection);

        if statement.is_err() {
            log::error!("{:?} {:?}",
                       DbEvent::DbQueryFailure,
                       statement.as_ref().err().unwrap());
            return Err(SimpleError::from(statement.err().unwrap()));
        }


        log::info!("{:?} {:?}", DbEvent::DbEntityCreation, store);
        Ok(store)
    }

    pub fn update(&self, store: &Store) -> Result<(), SimpleError> {

        let connection =
            self.db_connection.establish_connection()?;

        let filter = id.eq(store.id);

        let result =
            diesel::update(schema::stores::table.filter(filter))
                .set(store).execute(&connection);

        if result.is_err() {
            log::error!("{:?} {:?}",
                       DbEvent::DbQueryFailure,
                       result.as_ref().err().unwrap());
            return Err(SimpleError::from(result.err().unwrap()));
        }

        log::info!("{:?} {:?}",
                   DbEvent::DbEntityUpdate,
                   store);

        Ok(())
    }
}
