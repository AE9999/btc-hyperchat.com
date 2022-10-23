use actix_web::HttpResponse;
use actix_web::http::StatusCode;
use actix_web::web::Data;
use crate::AppContext;
use std::fmt;
use actix_web_middleware_keycloak_auth::StandardKeycloakClaims;
use crate::web::model::settings::btc_pay::btc_pay_config::BtcPayConfig;
use crate::web::service::btc_chat::my_received_btc_chats::find_user_by_keycloak_id::find_user_by_keycloak_id;

pub async fn get_btc_pay_config(claims: StandardKeycloakClaims,
                                context: Data<AppContext>) -> HttpResponse {

    log::info!("{:?} ",
               LogEvents::GetBtcPayConfigStart);

    let result =
        do_get_btc_pay_config(&claims, &context).await;

    match result {
        Ok(btcpay_config) => {
            log::info!("{:?}",
                        LogEvents::GetBtcPayConfigOk);
            HttpResponse::Ok().json(btcpay_config)
        },
        Err(error) => {
            log::error!("{:?} {:?} ",
                        LogEvents::GetBtcPayConfigError, error.1.clone());
            HttpResponse::build(error.0.clone())
                .body(LogEvents::GetBtcPayConfigError.to_string())
        }
    }
}

async fn do_get_btc_pay_config(claims: &StandardKeycloakClaims,
                                context: &Data<AppContext>) -> Result<BtcPayConfig,
                                                                (StatusCode, String)> {

    let user =
        find_user_by_keycloak_id(&claims.sub,
                                 context)?;

    let btc_pay_config =
        BtcPayConfig::new(user.email,
                 user.initial_btc_password,
            context.config.btc_pay_config.base_path.clone());

    Ok(btc_pay_config)
}

#[derive(Debug)]
pub enum LogEvents {
    GetBtcPayConfigStart,
    GetBtcPayConfigError,
    GetBtcPayConfigOk,
}

impl fmt::Display for LogEvents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogEvents::GetBtcPayConfigStart
            => write!(f, "GetBtcPayConfigStart"),
            LogEvents::GetBtcPayConfigError
            => write!(f, "GetBtcPayConfigError"),
            LogEvents::GetBtcPayConfigOk
            => write!(f, "GetBtcPayConfigOk"),
        }
    }
}
