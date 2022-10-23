use actix_web::http::StatusCode;
use btcpay_rust::models::StoreData;
use uuid::Uuid;
use crate::AppContext;
use crate::data::model::user::User;
use crate::web::service::user::create_store_in_db_from_user_and_store_data::create_store_in_db_from_user_and_store_data;
use std::fmt;
use actix_web::web::Data;
use crate::web::service::user::setup_btc_pay_accounts::create_btc_pay_store::create_btc_pay_store;
use crate::web::service::user::setup_btc_pay_accounts::setup_btc_pay_user_accounts::setup_btc_pay_user_accounts;
use crate::web::service::user::setup_btc_pay_accounts::setup_wallets_for_btc_pay_store::setup_wallets_for_btc_pay_store;
use crate::web::service::user::setup_btc_pay_accounts::setup_webhooks_for_btc_pay_store::setup_webhooks_for_btc_pay_store;

pub async fn setup_btc_pay_account(user : &User,
                                   keycloak_id: &Uuid,
                                   context: &Data<AppContext>) -> Result<StoreData, (StatusCode, String)> {

    log::info!("{:?}, {:?}, {:?}",
                LogEvents::SetupBtcPayAccountsStart,
                user,
                keycloak_id);

    let store_data = create_btc_pay_store(keycloak_id, context).await?;

    create_store_in_db_from_user_and_store_data(&user,
                                                &store_data,
                                                &context)?;

    let store_id =
        store_data.id
                  .as_ref()
                  .expect("There should be an id");

    setup_wallets_for_btc_pay_store(store_id.as_str(), context).await?;

    setup_webhooks_for_btc_pay_store(store_id.as_str(), context).await?;

    if context.config.btc_pay_config.register_new_user_accounts {
        setup_btc_pay_user_accounts(store_id.as_str(),
                                    user,
                                    context).await?;
    }

    log::info!("{:?}",
                LogEvents::SetupBtcPayAccountsOk);

    Ok(store_data)
}

#[derive(Debug)]
enum LogEvents {
    SetupBtcPayAccountsStart,
    SetupBtcPayAccountsOk,
}

impl fmt::Display for LogEvents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogEvents::SetupBtcPayAccountsStart
            => write!(f, "SetupBtcPayAccountsStart"),
            LogEvents::SetupBtcPayAccountsOk
            => write!(f, "SetupBtcPayAccountsOk"),
        }
    }
}







