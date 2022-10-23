use clap::{App, Arg, ArgMatches};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct WebsocketConfig {
    pub heartbeat_interval: u64,
    pub client_timeout: u64,
}

impl WebsocketConfig {
    pub fn init_config<'a>(app: App<'a, 'a>) -> App<'a, 'a> {
        app.arg(Arg::with_name("web-socket-heartbeat-interval")
                .long("web-socket-heartbeat-interval")
                .takes_value(true)
                .required(true)
                .env("BTC_BACKEND_WEB_SOCKET_HEARTBEAT_INTERVAL")
                .help("web-socket heartbeat interval"))
            .arg(Arg::with_name("web-socket-client-timeout")
                .long("web-socket-client-timeout")
                .takes_value(true)
                .required(true)
                .env("BTC_BACKEND_WEB_SOCKET_CLIENT_TIMEOUT")
                .help("web-socket client timeout"))
    }

    pub fn new(arg_matches: &ArgMatches) -> Self {
        let heartbeat_interval: u64 =
            arg_matches.value_of("web-socket-heartbeat-interval")
                .unwrap()
                .parse()
                .unwrap();

        let client_timeout: u64 =
            arg_matches.value_of("web-socket-client-timeout")
                .unwrap()
                .parse()
                .unwrap();

        WebsocketConfig {
            heartbeat_interval,
            client_timeout,
        }
    }
}
