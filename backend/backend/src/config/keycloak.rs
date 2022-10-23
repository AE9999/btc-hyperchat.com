use clap::{App, Arg, ArgMatches};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct  KeycloakConfig {
    pub public_key_file: String,
    pub url: String,
    pub username: String,
    pub password: String,
    pub realm: String,
}

impl KeycloakConfig {
    pub fn init_config<'a>(app: App<'a, 'a>) -> App<'a, 'a> {
        app.arg(Arg::with_name("keycloak-url")
                .long("keycloak-url")
                .takes_value(true)
                .required(true)
                .env("BTC_BACKEND_KEYCLOAK_URL")
                .help("keycloak url"))
           .arg(Arg::with_name("keycloak-username")
                .long("keycloak-username")
                .takes_value(true)
                .required(true)
                .env("BTC_BACKEND_KEYCLOAK_USERNAME")
                .help("keycloak username"))
           .arg(Arg::with_name("keycloak-password")
                .long("keycloak-password")
                .takes_value(true)
                .required(true)
                .env("BTC_BACKEND_KEYCLOAK_PASSWORD")
                .help("keycloak password"))
           .arg(Arg::with_name("keycloak-realm")
                .long("keycloak-realm")
                .takes_value(true)
                .required(true)
                .env("BTC_BACKEND_KEYCLOAK_REALM")
                .help("keycloak btc-chat realm"))
           .arg(Arg::with_name("keycloak-public-key-file")
                .long("keycloak-public-key-file")
                .takes_value(true)
                .required(true)
                .env("BTC_BACKEND_KEYCLOAK_PUBLIC_KEY_FILE")
                .help("keycloak public key file"))
    }

    pub fn new(arg_matches: &ArgMatches) -> Self {
        let keycloak_url =
            arg_matches.value_of("keycloak-url")
                .unwrap()
                .to_string();
        let keycloak_username =
            arg_matches.value_of("keycloak-username")
                .unwrap()
                .to_string();
        let keycloak_password =
            arg_matches.value_of("keycloak-password")
                .unwrap()
                .to_string();
        let keycloak_realm =
            arg_matches.value_of("keycloak-realm")
                .unwrap()
                .to_string();
        let keycloak_public_key_file =
            arg_matches.value_of("keycloak-public-key-file")
                .unwrap()
                .to_string();

        KeycloakConfig {
            url: keycloak_url,
            username: keycloak_username,
            password: keycloak_password,
            realm: keycloak_realm,
            public_key_file: keycloak_public_key_file,
        }
    }
}
