use actix_web::HttpResponse;
use actix_web::http::StatusCode;
use actix_web::web::Data;
use crate::AppContext;
use std::fmt;
use actix_web_middleware_keycloak_auth::StandardKeycloakClaims;

pub async fn reset_btc_pay_password(claims: StandardKeycloakClaims,
                                    context: Data<AppContext>) -> HttpResponse {

    log::info!("{:?} ",
               LogEvents::ResetBtcPayPasswordStart);

    let result =
        do_reset_btc_pay_password(&claims, &context).await;

    match result {
        Ok(_) => {
            log::info!("{:?}",
                        LogEvents::ResetBtcPayPasswordStartOk);
            HttpResponse::Ok().finish()
        },
        Err(error) => {
            log::error!("{:?} {:?} ",
                        LogEvents::ResetBtcPayPasswordError, error.1.clone());
            HttpResponse::build(error.0.clone())
                .body(LogEvents::ResetBtcPayPasswordError.to_string())
        }
    }
}

async fn do_reset_btc_pay_password(_claims: &StandardKeycloakClaims,
                                   _context: &Data<AppContext>) -> Result<(),
                                   (StatusCode, String)> {

    Ok(())
}

#[derive(Debug)]
pub enum LogEvents {
    ResetBtcPayPasswordStart,
    ResetBtcPayPasswordError,
    ResetBtcPayPasswordStartOk,
}

impl fmt::Display for LogEvents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogEvents::ResetBtcPayPasswordStart
            => write!(f, "ResetBtcPayPasswordStart"),
            LogEvents::ResetBtcPayPasswordError
            => write!(f, "ResetBtcPayPasswordStartError"),
            LogEvents::ResetBtcPayPasswordStartOk
            => write!(f, "ResetBtcPayPasswordStartOk"),
        }
    }
}
