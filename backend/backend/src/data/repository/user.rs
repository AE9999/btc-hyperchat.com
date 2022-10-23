use simple_error::SimpleError;
use uuid::Uuid;
use chrono::Utc;
use diesel::prelude::*;
use crate::data::repository::db_connection::DBConnection;
use crate::data::model::user::User;
use crate::data::schema::users::*;
use crate::data::schema::users::dsl::*;
use crate::data::schema;
use crate::data::repository::db_event::DbEvent;

#[derive(Clone, Debug)]
pub struct UserRepository {
    db_connection: DBConnection
}

impl UserRepository {
    pub fn new(db_connection: &DBConnection) -> Self {
        UserRepository {
            db_connection: db_connection.clone()
        }
    }

    pub fn find_by_pk_and_is_active(&self, id_: &Uuid) ->  Result<Option<User>,
                                                    SimpleError> {

        let connection =
            self.db_connection.establish_connection()?;

        let result: QueryResult<Vec<User>> =
            users.filter(schema::users::columns::id.eq(id_)
                    .and(schema::users::columns::active.eq(true)))
                 .load::<User>(&connection);

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

    pub fn find_by_keycloak_id_and_is_active(&self, keycloak_id_: &Uuid) -> Result<Option<User>,
                                                              SimpleError> {

        let connection =
            self.db_connection.establish_connection()?;

        let result : QueryResult<Vec<User>> =
            users.filter(schema::users::columns::keycloak_id.eq(keycloak_id_)
                                    .and(schema::users::columns::active.eq(true)))
                 .load::<User>(&connection);

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

    pub fn find_by_username_and_is_active(&self,
                                          username_: &str) -> Result<Option<User>, SimpleError> {

        // find_by_username

        let connection =
            self.db_connection.establish_connection()?;

        let result : QueryResult<Vec<User>> =
            users.filter(schema::users::columns::username.eq(username_)
                                    .and(schema::users::columns::active.eq(true)))
                 .load::<User>(&connection);

        if result.is_err() {
            log::error!("{:?} {:?}",
                       DbEvent::DbQueryFailure,
                       result.as_ref().err().unwrap());
            return Err(SimpleError::from(result.err().unwrap()));
        }

        let mut result = result.unwrap();

        if result.len() > 1 {
            log::error!("{:?}, Expected at most 1 result got: {:?}",
                       DbEvent::DbConsistencyFailure,
                       result.len());
            return Err(SimpleError::new(DbEvent::DbConsistencyFailure.to_string()));
        }

        Ok(if result.is_empty() { None }
           else { Some(result.remove(0)) })
    }

    pub fn find_with_username_in_and_is_active(&self,
                                               usernames: &Vec<&str>) -> Result<Vec<User>, SimpleError> {
        let connection =
            self.db_connection.establish_connection()?;

        let result : QueryResult<Vec<User>> =
            users.filter(schema::users::columns::username.eq_any(usernames)
                                    .and(schema::users::columns::active.eq(true)))
                .load::<User>(&connection);

        if result.is_err() {
            log::error!("{:?} {:?}",
                       DbEvent::DbQueryFailure,
                       result.as_ref().err().unwrap());
            return Err(SimpleError::from(result.err().unwrap()));
        }

        let result = result.unwrap();

        Ok(result)
    }

    pub fn find_like_username_and_is_active_with_limit(&self,
                                                       input: &str,
                                                       limit: i64) -> Result<Vec<User>, SimpleError> {
        let connection =
            self.db_connection.establish_connection()?;

        let sql_like =  "%".to_owned() + input + "%";

        let result : QueryResult<Vec<User>> =
            users.filter(schema::users::columns::username.ilike(sql_like.as_str())
                                    .and(schema::users::columns::active.eq(true)))
                .limit(limit)
                .order_by(schema::users::columns::username)
                .load::<User>(&connection);

        if result.is_err() {
            log::error!("{:?} {:?}",
                       DbEvent::DbQueryFailure,
                       result.as_ref().err().unwrap());
            return Err(SimpleError::from(result.err().unwrap()));
        }

        let result = result.unwrap();
        Ok(result)
    }

    pub fn create(&self,
                  username_: String,
                  email_: String,
                  initial_btc_password_: String,
                  keycloak_id_: Uuid,
                  active_: bool
    ) -> Result<User, SimpleError> {

        let now  = Utc::now();
        let user = User::new(Uuid::new_v4(),
                                     username_,
                                       email_,
                             initial_btc_password_,
                                       keycloak_id_,
                                       active_,
                             now,
                             now);

        let connection =
            self.db_connection.establish_connection()?;

        let result =
            diesel::insert_into(schema::users::table)
                   .values(&user)
                   .execute(&connection);

        if result.is_err() {
            log::error!("{:?} {:?}",
                       DbEvent::DbQueryFailure,
                       result.as_ref().err().unwrap());
            return Err(SimpleError::from(result.err().unwrap()));
        }

        log::info!("{:?} {:?}", DbEvent::DbEntityCreation, user);

        Ok(user)
    }

    pub fn update(&self, user: &User) -> Result<(), SimpleError> {

        let connection =
            self.db_connection.establish_connection()?;

        let filter = id.eq(user.id);

        let result =
            diesel::update(schema::users::table.filter(filter))
                   .set(user).execute(&connection);

        if result.is_err() {
            log::error!("{:?} {:?}",
                       DbEvent::DbQueryFailure,
                       result.as_ref().err().unwrap());
            return Err(SimpleError::from(result.err().unwrap()));
        }

        log::info!("{:?} {:?}", DbEvent::DbEntityUpdate, user);

        Ok(())
    }


}
