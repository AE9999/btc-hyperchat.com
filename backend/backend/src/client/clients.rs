use crate::config::config::Config;
use btcpay_rust::apis::configuration::Configuration as BtcPayConfiguration;

#[derive(Clone)]
pub struct Clients {
    pub btc_pay_configuration: BtcPayConfiguration ,
}

impl Clients {
    // For BTC Pay we will need https://jorijn.com/installing-nginx-reverse-proxy-with-ssl-certificate-umbrel-btcpayserver/

    pub fn new(config: &Config) -> Self {
        let btc_pay_configuration = Clients::btc_pay_configuration(config);

        Clients {
            btc_pay_configuration,
        }
    }

    fn btc_pay_configuration(config: &Config) -> BtcPayConfiguration {
        let mut btc_pay_configuration = BtcPayConfiguration::new();
        btc_pay_configuration.base_path = config.btc_pay_config.base_path.clone();
        btc_pay_configuration.user_agent = Some(config.btc_pay_config.user_agent.clone());
        let basic_auth = (config.btc_pay_config.username.clone(),
                          Some( config.btc_pay_config.password.clone() ) );
        btc_pay_configuration.basic_auth = Some(basic_auth);
        btc_pay_configuration
    }

}
