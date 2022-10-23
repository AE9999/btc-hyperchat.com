use clap::{App, Arg, ArgMatches};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct  InvoiceConfig {
    pub min_amount: u32,
    pub max_amount: u32,
    pub accepted_currencies: Vec<String>,
}

impl InvoiceConfig {
    pub fn init_config<'a>(app: App<'a, 'a>) -> App<'a, 'a> {
        app.arg(Arg::with_name("invoice-min-amount")
                .long("invoice-min-amount")
                .takes_value(true)
                .required(true)
                .env("BTC_BACKEND_INVOICE_MIN_AMOUNT")
                .help("invoice min amount"))
            .arg(Arg::with_name("invoice-max-amount")
                .long("invoice-max-amount")
                .takes_value(true)
                .required(true)
                .env("BTC_BACKEND_INVOICE_MAX_AMOUNT")
                .help("invoice max amount"))
            .arg(Arg::with_name("invoice-accepted-currencies")
                .long("invoice-accepted-currencies")
                .takes_value(true)
                .required(true)
                .env("BTC_BACKEND_INVOICE_ACCEPTED_CURRENCIES")
                .help("accepted currencies"))
            .arg(Arg::with_name("invoice-default-minimal-payment-status-code-before-callback")
                .long("invoice-default-minimal-payment-status-code-before-callback")
                .takes_value(true)
                .required(true)
                .env("BTC_BACKEND_INVOICE_DEFAULT_MINIMAL_PAYMENT_STATUS_CODE_BEFORE_CALLBACK")
                .help("invoice default minimal payment status code before callback"))
    }

    pub fn new(arg_matches: &ArgMatches) -> Self {
        let min_amount: u32 =
            arg_matches.value_of("invoice-min-amount")
                       .unwrap()
                       .parse()
                       .unwrap();
        let max_amount: u32 =
            arg_matches.value_of("invoice-max-amount")
                       .unwrap()
                       .parse()
                       .unwrap();
        let accepted_currencies: Vec<String> =
            arg_matches.value_of("invoice-accepted-currencies")
                       .unwrap()
                       .split(",")
                       .map(|s| String::from(s))
                       .collect();

        InvoiceConfig {
            min_amount,
            max_amount,
            accepted_currencies,
        }
    }
}
