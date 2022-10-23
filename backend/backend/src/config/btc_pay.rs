use clap::{App, Arg, ArgMatches};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct BTCPayConfig {
    pub base_path: String,
    pub username: String,
    pub password: String,
    pub user_agent: String,
    pub callback_url_base: String,
    pub callback_secret: String,
    pub register_new_user_accounts : bool,
}

impl BTCPayConfig {
    pub fn init_config<'a>(app: App<'a, 'a>) -> App<'a, 'a> {
        app.arg(Arg::with_name("btc-pay-base-path")
                .long("btc-pay-base-path")
                .takes_value(true)
                .required(true)
                .env("BTC_BACKEND_BTC_PAY_BASE_PATH")
                .help("btc pay base path"))
           .arg(Arg::with_name("btc-pay-username")
                .long("btc-pay-username")
                .takes_value(true)
                .required(true)
                .env("BTC_BACKEND_BTC_PAY_USERNAME")
                .help("btc pay username"))
            .arg(Arg::with_name("btc-pay-password")
                .long("btc-pay-password")
                .takes_value(true)
                .required(true)
                .env("BTC_BACKEND_BTC_PAY_PASSWORD")
                .help("btc pay password"))
            .arg(Arg::with_name("btc-pay-user-agent")
                .long("btc-pay-user-agent")
                .takes_value(true)
                .required(true)
                .env("BTC_BACKEND_BTC_PAY_USER_AGENT")
                .help("btc pay user agent"))
            .arg(Arg::with_name("btc-pay-callback-url-base")
                .long("btc-pay-callback-url-base")
                .takes_value(true)
                .required(true)
                .env("BTC_BACKEND_BTC_PAY_CALLBACK_URL_BASE")
                .help("btc pay callback url"))
            .arg(Arg::with_name("btc-pay-callback-secret")
                .long("btc-pay-callback-secret")
                .takes_value(true)
                .required(true)
                .env("BTC_BACKEND_BTC_PAY_CALLBACK_SECRET")
                .help("btc pay callback secret"))
            .arg(Arg::with_name("btc-pay-register-new-user-accounts")
                .long("btc-pay-register-new-user-accounts")
                .takes_value(true)
                .required(true)
                .env("BTC_BACKEND_BTC_PAY_REGISTER_NEW_USER_ACCOUNTS")
                .help("btc pay register new user accounts"))

    }

    pub fn new(arg_matches: &ArgMatches) -> Self {
        let base_path =
            arg_matches.value_of("btc-pay-base-path")
                       .unwrap()
                       .to_string();
        let username =
            arg_matches.value_of("btc-pay-username")
                       .unwrap()
                       .to_string();
        let password =
            arg_matches.value_of("btc-pay-password")
                       .unwrap()
                       .to_string();
        let user_agent =
            arg_matches.value_of("btc-pay-user-agent")
                       .unwrap()
                       .to_string();
        let callback_url_base =
            arg_matches.value_of("btc-pay-callback-url-base")
                       .unwrap()
                       .to_string();
        let callback_secret =
            arg_matches.value_of("btc-pay-callback-secret")
                       .unwrap()
                       .to_string();
        let register_new_user_accounts : bool =
            arg_matches.value_of("btc-pay-register-new-user-accounts")
                .unwrap()
                .to_string()
                .parse()
                .unwrap();

        BTCPayConfig {
            base_path,
            username,
            password,
            user_agent,
            callback_url_base,
            callback_secret,
            register_new_user_accounts,
        }
    }
}
