use clap::{App, Arg, ArgMatches};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct DatabaseConfig {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: i32,
    pub database: String,
    pub max_pool_size: u32,
}

impl DatabaseConfig {
    pub fn init_config<'a>(app: App<'a, 'a>) -> App<'a, 'a> {
        app.arg(Arg::with_name("postgres-username")
                .long("postgres-username")
                .env("BTC_BACKEND_POSTGRES_USERNAME")
                .takes_value(true)
                .required(true)
                .help("postgres username"))
            .arg(Arg::with_name("postgres-password")
                .long("postgres-password")
                .env("BTC_BACKEND_POSTGRES_PASSWORD")
                .takes_value(true)
                .required(true)
                .help("postgres password"))
            .arg(Arg::with_name("postgres-host")
                .long("postgres-host")
                .env("BTC_BACKEND_POSTGRES_HOST")
                .takes_value(true)
                .required(true)
                .help("postgres host"))
            .arg(Arg::with_name("postgres-port")
                .long("postgres-port")
                .env("BTC_BACKEND_POSTGRES_PORT")
                .takes_value(true)
                .default_value("5432")
                .required(false)
                .help("postgres port"))
            .arg(Arg::with_name("postgres-database")
                .long("postgres-database")
                .env("BTC_BACKEND_POSTGRES_DATABASE")
                .takes_value(true)
                .required(true)
                .help("postgres database"))
            .arg(Arg::with_name("postgres-max-pool-size")
                .long("postgres-max-pool-size")
                .env("BTC_BACKEND_POSTGRES_MAX_POOL_SIZE")
                .takes_value(true)
                .required(true)
                .help("postgres max pool url"))
    }

    pub fn url(&self) -> String {
        // postgres://postgres:password@localhost:5432/btc_chat
        format!("postgres://{}:{}@{}:{}/{}",
                self.username,
                self.password,
                self.host,
                self.port,
                self.database)

    }

    pub fn new(arg_matches: &ArgMatches) -> Self {

        let username =
            arg_matches.value_of("postgres-username")
                       .unwrap()
                       .to_string();

        let password =
            arg_matches.value_of("postgres-password")
                       .unwrap()
                       .to_string();

        let host =
            arg_matches.value_of("postgres-host")
                       .unwrap()
                       .to_string();

        let port: i32 =
            arg_matches.value_of("postgres-port")
                       .unwrap()
                       .to_string()
                       .parse()
                       .unwrap();

        let database =
            arg_matches.value_of("postgres-database")
                       .unwrap()
                       .to_string();

        let max_pool_size: u32 =
            arg_matches.value_of("postgres-max-pool-size")
                       .unwrap()
                       .to_string()
                       .parse()
                       .unwrap();

        DatabaseConfig {
            username,
            password,
            host,
            port,
            database,
            max_pool_size,
        }
    }
}
