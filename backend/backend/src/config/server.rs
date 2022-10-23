use clap::{App, Arg, ArgMatches};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct  ServerConfig {
    pub host: String,
    pub port: u16,
    pub callback_to_user_timeout: u64,
    pub cors_allowed_origin: String,
}

impl ServerConfig {
    pub fn init_config<'a>(app: App<'a, 'a>) -> App<'a, 'a> {
        app.arg(Arg::with_name("server-port")
                .long("server-port")
                .takes_value(true)
                .required(true)
                .env("BTC_BACKEND_SERVER_PORT")
                .help("server port"))
           .arg(Arg::with_name("server-host")
                .long("server-host")
                .takes_value(true)
                .required(true)
                .env("BTC_BACKEND_SERVER_HOST")
                .help("server host"))
            .arg(Arg::with_name("server-callback-to-user-timeout")
                .long("server-callback-to-user-timeout")
                .takes_value(true)
                .required(true)
                .env("BTC_BACKEND_SERVER_CALLBACK_TO_USER_TIMEOUT")
                .help("server callback to user timeout"))
            .arg(Arg::with_name("server-cors-allowed-origin")
                .long("server-cors-allowed-origin")
                .takes_value(true)
                .required(true)
                .env("BTC_BACKEND_SERVER_CORS_ALLOWED_ORIGIN")
                .help("server cors allowed origin"))
    }

    pub fn new(arg_matches: &ArgMatches) -> Self {
        let port: u16 =
            arg_matches.value_of("server-port")
                       .unwrap()
                       .parse()
                       .unwrap();
        let host  =
            arg_matches.value_of("server-host")
                       .unwrap()
                       .parse()
                       .unwrap();

        let callback_to_user_timeout =
            arg_matches.value_of("server-callback-to-user-timeout")
                       .unwrap()
                       .parse()
                       .unwrap();

        let cors_allowed_origin =
            arg_matches.value_of("server-cors-allowed-origin")
                       .unwrap()
                       .parse()
                       .unwrap();
        ServerConfig {
            port,
            host,
            callback_to_user_timeout,
            cors_allowed_origin,
        }
    }
}
