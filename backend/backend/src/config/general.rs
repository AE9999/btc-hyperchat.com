use clap::{App, Arg, ArgMatches};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct  GeneralConfig {
    pub debug_level: String,
    pub profile_autocomplete_max_results :i64,
    pub user_registration_username: String,
    pub user_registration_password: String,
    pub top_profiles: String,
}

impl GeneralConfig {
    pub fn init_config<'a>(app: App<'a, 'a>) -> App<'a, 'a> {
        app.arg(Arg::with_name("debug-level")
                .long("debug-level")
                .takes_value(true)
                .required(false)
                .env("BTC_BACKEND_DEBUG_LEVEL")
                .help("debug level"))
           .arg(Arg::with_name("user-registration-username")
                .long("user-registration-username")
                .takes_value(true)
                .required(true)
                .env("BTC_BACKEND_REGISTRATION_USERNAME")
                .help("user registration username"))
            .arg(Arg::with_name("user-registration-password")
                .long("user-registration-password")
                .takes_value(true)
                .required(true)
                .env("BTC_BACKEND_USER_REGISTRATION_PASSWORD")
                .help("user registration password"))
            .arg(Arg::with_name("profile-autocomplete-max-results")
                .long("profile-autocomplete-max-results")
                .takes_value(true)
                .required(true)
                .env("BTC_BACKEND_PROFILE_AUTOCOMPLETE_MAX_RESULTS")
                .help("profile autocomplete max results"))
            .arg(Arg::with_name("top-profiles")
                .long("top-profiles")
                .takes_value(true)
                .required(true)
                .env("BTC_BACKEND_TOP_PROFILES")
                .help("top profiles"))
    }

    pub fn new(arg_matches: &ArgMatches) -> Self {
        let debug_level =
            arg_matches.value_of("debug-level")
                       .unwrap()
                       .to_string();
        let user_registration_username =
            arg_matches.value_of("user-registration-username")
                       .unwrap()
                       .to_string();
        let user_registration_password =
            arg_matches.value_of("user-registration-password")
                       .unwrap()
                       .to_string();
        let profile_autocomplete_max_results: i64 =
            arg_matches.value_of("profile-autocomplete-max-results")
                       .unwrap()
                       .to_string()
                       .parse()
                       .unwrap();
        let top_profiles =
            arg_matches.value_of("top-profiles")
                       .unwrap()
                       .to_string()
                       .parse()
                       .unwrap();

        GeneralConfig {
            debug_level,
            user_registration_username,
            user_registration_password,
            profile_autocomplete_max_results,
            top_profiles
        }
    }
}
