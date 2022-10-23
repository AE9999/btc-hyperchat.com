use actix_web::HttpResponse;
use actix_web::http::StatusCode;
use actix_web::web::Data;
use crate::AppContext;
use std::fmt;
use actix_web_middleware_keycloak_auth::StandardKeycloakClaims;
use crate::web::service::btc_chat::my_received_btc_chats::find_user_by_keycloak_id::find_user_by_keycloak_id;
use crate::web::service::btc_chat::util::find_store_for_user_id::find_store_for_owner_id;
use crate::web::service::settings::delete_account::delete_user_from_btc_pay::delete_user_from_btc_pay;
use crate::web::service::settings::delete_account::delete_user_from_database::delete_user_from_database;
use crate::web::service::settings::delete_account::delete_user_from_keycloak::delete_user_from_keycloak;

pub async fn delete_account(claims: StandardKeycloakClaims,
                                    context: Data<AppContext>) -> HttpResponse {

    log::info!("{:?} ",
               LogEvents::DeleteAccountStart);

    let result =
        do_delete_account(&claims, &context).await;

    match result {
        Ok(_) => {
            log::info!("{:?}",
                        LogEvents::DeleteAccountOk);
            HttpResponse::Ok().finish()
        },
        Err(error) => {
            log::error!("{:?} {:?} ",
                        LogEvents::DeleteAccountError, error.1.clone());
            HttpResponse::build(error.0.clone())
                .body(LogEvents::DeleteAccountError.to_string())
        }
    }
}

async fn do_delete_account(claims: &StandardKeycloakClaims,
                           context: &Data<AppContext>) -> Result<(), (StatusCode, String)> {

    let mut user =
        find_user_by_keycloak_id(&claims.sub,
                                 context)?;

    let store =
        find_store_for_owner_id(&user.id,
                                context)?;

    let ok =
        delete_user_from_btc_pay(&user.email,
                                 store.btcpay_store_id.as_str(),
                                  &context.clients.btc_pay_configuration).await;
    if ok.is_err() {
        log::error!("{:?} {:?} ",
                    LogEvents::DeleteAccountBtcPayError,
                    ok.err().unwrap());
    }

    let ok =
        delete_user_from_database(&mut user, context);

    if ok.is_err() {
        log::error!("{:?} {:?} ",
                    LogEvents::DeleteAccountDatabaseError,
                    ok.err().unwrap());
    }

    let ok =
        delete_user_from_keycloak(&claims.sub, context).await;
    if ok.is_err() {
        log::error!("{:?} {:?} ",
                    LogEvents::DeleteAccountKeycloakError,
                    ok.err().unwrap());
    }

    Ok(())
}

#[derive(Debug)]
pub enum LogEvents {
    DeleteAccountStart,
    DeleteAccountBtcPayError,
    DeleteAccountDatabaseError,
    DeleteAccountKeycloakError,
    DeleteAccountError,
    DeleteAccountOk,
}

impl fmt::Display for LogEvents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogEvents::DeleteAccountStart
                => write!(f, "DeleteAccountStart"),
            LogEvents::DeleteAccountBtcPayError
                => write!(f, "DeleteAccountBtcPayError"),
            LogEvents::DeleteAccountDatabaseError
                => write!(f, "DeleteAccountDatabaseError"),
            LogEvents::DeleteAccountKeycloakError
                => write!(f, "DeleteAccountKeycloakError"),
            LogEvents::DeleteAccountError
                => write!(f, "DeleteAccountError"),
            LogEvents::DeleteAccountOk
                => write!(f, "DeleteAccountOk"),
        }
    }
}
