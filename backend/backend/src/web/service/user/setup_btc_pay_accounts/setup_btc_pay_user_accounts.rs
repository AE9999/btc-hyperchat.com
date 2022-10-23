use actix_web::http::StatusCode;
use actix_web::web::Data;
use crate::AppContext;
use std::fmt;
use btcpay_rust::apis::stores_users_api::stores_add_store_user;
use btcpay_rust::apis::users_api::users_create_user;
use btcpay_rust::models::{StoreUserData, UsersCreateUserRequest};
use crate::data::model::user::User;

pub async fn setup_btc_pay_user_accounts(store_id: &str,
                                         user : &User,
                                         context: &Data<AppContext>) -> Result<(), (StatusCode, String)> {

    log::info!("{:?}, {:?}",
                LogEvents::SetupBtcPayUserAccountsStart,
                store_id);


    let mut user_create_user_request = UsersCreateUserRequest::new();
    user_create_user_request.is_administrator = Some(false);
    user_create_user_request.email = Some(user.email.clone());
    user_create_user_request.password = Some(user.initial_btc_password.clone()); // Fix this

    let application_user_data =
     users_create_user(&context.clients.btc_pay_configuration,
                       user_create_user_request).await;

    if application_user_data.is_err() {
        log::error!("{:?}, {:?}, {:?}",
                    LogEvents::SetupBtcPayUserAccountsCreateUserError,
                    application_user_data.as_ref().err().unwrap().to_string(),
                    &context.clients.btc_pay_configuration);
        return Err((StatusCode::INTERNAL_SERVER_ERROR,
                    LogEvents::SetupBtcPayUserAccountsCreateUserError.to_string()))
    }

    let application_user_data = application_user_data.unwrap();

    let mut store_user_data = StoreUserData::new();
    store_user_data.user_id = Some(application_user_data.id.as_ref().unwrap().clone());
    store_user_data.role = Some("Owner".to_string());

    let ok =
        stores_add_store_user(&context.clients.btc_pay_configuration,
                              store_id,
                              store_user_data).await;

    if ok.is_err() {
        log::error!("{:?}, {:?}, {:?}",
                    LogEvents::SetupBtcPayUserAccountsAddUserToStoresError,
                    ok.as_ref().err().unwrap().to_string(),
                    &context.clients.btc_pay_configuration);
        return Err((StatusCode::INTERNAL_SERVER_ERROR,
                    LogEvents::SetupBtcPayUserAccountsAddUserToStoresError.to_string()))
    }

    log::info!("{:?}",
                LogEvents::SetupBtcPayUserAccountsOk);

    Ok(())
}

#[derive(Debug)]
enum LogEvents {
    SetupBtcPayUserAccountsStart,
    SetupBtcPayUserAccountsCreateUserError,
    SetupBtcPayUserAccountsAddUserToStoresError,
    SetupBtcPayUserAccountsOk,
}

impl fmt::Display for LogEvents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogEvents::SetupBtcPayUserAccountsStart
                => write!(f, "SetupBtcPayUserAccountsStart"),
            LogEvents::SetupBtcPayUserAccountsCreateUserError
                => write!(f, "SetupBtcPayUserAccountsCreateUserError"),
            LogEvents::SetupBtcPayUserAccountsAddUserToStoresError
                => write!(f, "SetupBtcPayUserAccountsAddUserToStoresError"),
            LogEvents::SetupBtcPayUserAccountsOk
                => write!(f, "SetupBtcPayUserAccountsOk"),
        }
    }
}
