use diesel::pg::PgConnection;
use simple_error::SimpleError;
use crate::config::database::DatabaseConfig;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::r2d2::Pool;
use std::fmt;

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct DBConnection {
    database_config: DatabaseConfig,

    #[serde(skip_serializing, skip_deserializing)]
    pool:  Option<Pool<ConnectionManager<PgConnection>>>,
}

impl fmt::Debug for DBConnection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.database_config.fmt(f)
    }
}

impl DBConnection {

    pub fn new(database_config: &DatabaseConfig) -> Self {
        let manager =
            ConnectionManager::<PgConnection>::new(database_config.url().as_str());

        let pool =
            Pool::builder()
                .test_on_check_out(true)
                .max_size(database_config.max_pool_size)
                .build(manager)
                .expect("Could not build connection pool");

        DBConnection {
            database_config: database_config.clone(),
            pool: Some(pool),
        }
    }

    pub fn establish_connection(&self) ->
                                       Result<PooledConnection<ConnectionManager<PgConnection>>,
                                              SimpleError> {

        let connection =
            self.pool.as_ref().expect(LogEvents::UninitializedError.to_string().as_str()).get();

        if connection.is_err() {
            let error = connection.err().unwrap();
            log::error!("{:?} {:?}", LogEvents::ConnectionError, error);
            return Err(SimpleError::from(error))
        }

        let connection = connection.unwrap();

        Ok(connection)
    }
}

#[derive(Debug)]
enum LogEvents {
    ConnectionError,
    UninitializedError,
}

impl fmt::Display for LogEvents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogEvents::ConnectionError
                => write!(f, "ConnectionError"),
            LogEvents::UninitializedError
                => write!(f, "UninitializedError"),
        }
    }
}


