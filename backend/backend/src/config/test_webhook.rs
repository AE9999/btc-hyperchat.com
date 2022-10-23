use std::str::FromStr;
use clap::{App, Arg, ArgMatches};
use uuid::Uuid;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct TestWebhookConfig {
    pub id: Uuid,
    pub message : String,
    pub sender :String,
    pub amount_of_sats: i64,
    pub amount_in_fiat : i32,
    pub currency: String,
    pub invoice_id: String,
}

impl TestWebhookConfig {
    pub fn init_config<'a>(app: App<'a, 'a>) -> App<'a, 'a> {
        app.arg(Arg::with_name("test-webhook-id")
                .long("test-webhook-id")
                .takes_value(true)
                .required(true)
                .env("BTC_BACKEND_TEST_WEBHOOK_ID")
                .help("test-webhook id"))
            .arg(Arg::with_name("test-webhook-message")
                .long("test-webhook-message")
                .takes_value(true)
                .required(true)
                .env("BTC_BACKEND_TEST_WEBHOOK_MESSAGE")
                .help("test-webhook message"))
            .arg(Arg::with_name("test-webhook-sender")
                .long("test-webhook-sender")
                .takes_value(true)
                .required(true)
                .env("BTC_BACKEND_TEST_WEBHOOK_SENDER")
                .help("test-webhook sender"))
            .arg(Arg::with_name("test-webhook-amount-of-sats")
                .long("test-webhook-amount-of-sats")
                .takes_value(true)
                .required(true)
                .env("BTC_BACKEND_TEST_WEBHOOK_AMOUNT_OF_SATS")
                .help("test-webhook amount of sats"))
            .arg(Arg::with_name("test-webhook-amount-in-fiat")
                .long("test-webhook-amount-in-fiat")
                .takes_value(true)
                .required(true)
                .env("BTC_BACKEND_TEST_WEBHOOK_AMOUNT_IN_FIAT")
                .help("test-webhook amount in fiat"))
            .arg(Arg::with_name("test-webhook-currency")
                .long("test-webhook-currency")
                .takes_value(true)
                .required(true)
                .env("BTC_BACKEND_TEST_WEBHOOK_CURRENCY")
                .help("test-webhook currency"))
            .arg(Arg::with_name("test-webhook-invoice-id")
                .long("test-webhook-invoice-id")
                .takes_value(true)
                .required(true)
                .env("BTC_BACKEND_TEST_WEBHOOK_INVOICE_ID")
                .help("test-webhook invoice id"))


    }

    pub fn new(arg_matches: &ArgMatches) -> Self {
        let id: Uuid =
            Uuid::from_str(arg_matches.value_of("test-webhook-id")
                                             .unwrap())
                 .unwrap();

        let message : String =
            arg_matches.value_of("test-webhook-message")
                       .unwrap()
                       .to_string();

        let sender :String =
            arg_matches.value_of("test-webhook-sender")
                       .unwrap()
                       .to_string();

        let amount_of_sats: i64 =
            arg_matches.value_of("test-webhook-amount-of-sats")
                       .unwrap()
                       .parse()
                       .unwrap();

        let amount_in_fiat : i32 =
            arg_matches.value_of("test-webhook-amount-in-fiat")
                       .unwrap()
                       .parse()
                       .unwrap();

        let currency: String =
            arg_matches.value_of("test-webhook-currency")
                       .unwrap()
                       .to_string();

        let invoice_id: String =
            arg_matches.value_of("test-webhook-invoice-id")
                .unwrap()
                .to_string();

        TestWebhookConfig {
            id,
            message,
            sender,
            amount_of_sats,
            amount_in_fiat,
            currency,
            invoice_id,
        }
    }
}
