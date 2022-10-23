use actix_web::http::StatusCode;
use btcpay_rust::apis::stores_api::stores_delete_store;
use btcpay_rust::apis::configuration::Configuration as BtcPayConfiguration;
use std::fmt;
use btcpay_rust::apis::users_api::{users_delete_user, users_get_user};

pub async fn delete_user_from_btc_pay(store_id: &str,
                                      user_email: &str,
                                      btc_pay_configuration: &BtcPayConfiguration) ->
        Result<(), (StatusCode, String)> {

    log::info!("{:?}, {:?}, {:?}",
                LogEvents::DeleteUserFromBtcPayStart,
                store_id,
                user_email);

    let ok =
        users_get_user(btc_pay_configuration, user_email).await;

    let user_found: bool =
        match ok {
            Err(error) => {
                match error {
                    btcpay_rust::apis::Error::ResponseError(error) => {
                        if error.status.as_u16() == 404 as u16 {
                            log::info!("{:?}",
                                   LogEvents::DeleteUserFromBtcPayNoUserFound);
                            false
                        } else {
                            log::error!("{:?} {:?}",
                                        LogEvents::DeleteUserFromBtcPayRetrieveUserError,
                                        error);
                            return Err((StatusCode::INTERNAL_SERVER_ERROR,
                                        LogEvents::DeleteUserFromBtcPayRetrieveUserError.to_string()));
                        }
                    },
                    _ => {
                        log::error!("{:?} {:?}",
                                    LogEvents::DeleteUserFromBtcPayRetrieveUserError,
                                    error);
                        return Err((StatusCode::INTERNAL_SERVER_ERROR,
                                    LogEvents::DeleteUserFromBtcPayRetrieveUserError.to_string()));
                    }
                }
            },
            Ok(_) => {
                log::info!("{:?}",
                                   LogEvents::DeleteUserFromBtcPayUserFound);
                true
            }
        };

    if user_found {
        let ok =
            users_delete_user(btc_pay_configuration, user_email).await;

        if ok.is_err() {
            log::error!("{:?} {:?}",
                        LogEvents::DeleteUserFromBtcPayDeleteUserError,
                        ok.err().expect(LogEvents::DeleteUserFromBtcPayDeleteUserError
                                                  .to_string()
                                                  .as_str()));
            return Err((StatusCode::INTERNAL_SERVER_ERROR,
                        LogEvents::DeleteUserFromBtcPayDeleteUserError.to_string()));
        }
    }

    let ok =
        stores_delete_store(btc_pay_configuration, store_id).await;

    if ok.is_err() {
        log::error!("{:?} {:?}",
                     LogEvents::DeleteUserFromBtcPayDeleteStoreError,
                      ok.err().expect(LogEvents::DeleteUserFromBtcPayDeleteStoreError
                                                  .to_string()
                                                  .as_str()));
        return Err((StatusCode::INTERNAL_SERVER_ERROR,
                    LogEvents::DeleteUserFromBtcPayDeleteStoreError.to_string()));
    }

    log::info!("{:?}",
                LogEvents::DeleteUserFromBtcPayOk);

    Ok(())
}


#[derive(Debug)]
enum LogEvents {
    DeleteUserFromBtcPayStart,
    DeleteUserFromBtcPayUserFound,
    DeleteUserFromBtcPayNoUserFound,
    DeleteUserFromBtcPayRetrieveUserError,
    DeleteUserFromBtcPayDeleteUserError,
    DeleteUserFromBtcPayDeleteStoreError,
    DeleteUserFromBtcPayOk,
}

impl fmt::Display for LogEvents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogEvents::DeleteUserFromBtcPayStart
                => write!(f, "DeleteUserFromBtcPayStart"),
            LogEvents::DeleteUserFromBtcPayUserFound
                => write!(f, "DeleteUserFromBtcPayUserFound"),
            LogEvents::DeleteUserFromBtcPayNoUserFound
                => write!(f, "DeleteUserFromBtcPayNoUserFound"),
            LogEvents::DeleteUserFromBtcPayRetrieveUserError
                => write!(f, "DeleteUserFromBtcPayRetrieveUserError"),
            LogEvents::DeleteUserFromBtcPayDeleteUserError
                => write!(f, "DeleteUserFromBtcPayDeleteUserError"),
            LogEvents::DeleteUserFromBtcPayDeleteStoreError
                => write!(f, "DeleteUserFromBtcPayDeleteStoreError"),
            LogEvents::DeleteUserFromBtcPayOk
                => write!(f, "DeleteUserFromBtcPayOk"),
        }
    }
}
