use clap::{App, ArgMatches};
use crate::config::btc_pay::BTCPayConfig;
use crate::config::database::DatabaseConfig;
use crate::config::general::GeneralConfig;
use crate::config::invoice::InvoiceConfig;
use crate::config::keycloak::KeycloakConfig;
use crate::config::server::ServerConfig;
use crate::config::test_webhook::TestWebhookConfig;
use crate::config::websocket::WebsocketConfig;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Config {
    pub btc_pay_config: BTCPayConfig,
    pub database_config: DatabaseConfig,
    pub keycloak_config: KeycloakConfig,
    pub general_config: GeneralConfig,
    pub invoice_config: InvoiceConfig,
    pub websocket_config: WebsocketConfig,
    pub test_webhook_config: TestWebhookConfig,
    pub server_config: ServerConfig,
}

impl Config {
    pub fn new(arg_matches: &ArgMatches) -> Self {
        Config {
            btc_pay_config: BTCPayConfig::new(arg_matches),
            database_config: DatabaseConfig::new(arg_matches),
            keycloak_config: KeycloakConfig::new(arg_matches),
            general_config: GeneralConfig::new(arg_matches),
            invoice_config: InvoiceConfig::new(arg_matches),
            websocket_config: WebsocketConfig::new(arg_matches),
            test_webhook_config: TestWebhookConfig::new(arg_matches),
            server_config: ServerConfig::new(arg_matches),
        }
    }
}


pub fn config_from_cmd() -> Config {
    let args = wild::args_os();
    let args = argfile::expand_args_from(
        args,
        argfile::parse_fromfile,
        argfile::PREFIX,
    ).unwrap();

    let app = App::new("BTC-CHAT")
        .version("0.1.0")
        .author("AE")
        .about("BTC-CHAT");
    let app = BTCPayConfig::init_config(app);
    let app = DatabaseConfig::init_config(app);
    let app = KeycloakConfig::init_config(app);
    let app = GeneralConfig::init_config(app);
    let app = InvoiceConfig::init_config(app);
    let app = WebsocketConfig::init_config(app);
    let app = TestWebhookConfig::init_config(app);
    let app = ServerConfig::init_config(app);
    let arg_matches = app.get_matches_from(args);

    Config::new(&arg_matches)
}
