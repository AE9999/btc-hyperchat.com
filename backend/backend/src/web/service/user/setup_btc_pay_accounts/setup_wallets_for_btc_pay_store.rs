use std::fmt;
use actix_web::http::StatusCode;
use actix_web::web::Data;
use btcpay_rust::apis::store_payment_methods_on_chain_api::{store_on_chain_payment_methods_generate_on_chain_wallet};
use btcpay_rust::models::GenerateOnChainWalletRequest;
use crate::AppContext;

pub async fn setup_wallets_for_btc_pay_store(store_id: &str,
                                         context: &Data<AppContext>) -> Result<(), (StatusCode, String)> {

    log::info!("{:?}, {:?}",
                LogEvents::SetupWalletsForBtcPayStoreStart,
                store_id);

    let crypto_code = "BTC";
    let mut generate_on_chain_wallet_request =
        GenerateOnChainWalletRequest::new();

    generate_on_chain_wallet_request.save_private_keys = Some(true);

    // https://docs.btcpayserver.org/API/Greenfield/v1/#operation/StoreOnChainPaymentMethods_GenerateOnChainWallet
    let on_chain_payment_method_data_with_sensitive_data =
        store_on_chain_payment_methods_generate_on_chain_wallet(&context.clients.btc_pay_configuration,
                                                                store_id,
                                                                crypto_code,
                                                                generate_on_chain_wallet_request).await;

    if on_chain_payment_method_data_with_sensitive_data.is_err() {
        log::error!("{:?}, {:?}, {:?}",
                    LogEvents::SetupWalletsForBtcPayStoreError,
                    on_chain_payment_method_data_with_sensitive_data.as_ref().err().unwrap().to_string(),
                    &context.clients.btc_pay_configuration);
        return Err((StatusCode::INTERNAL_SERVER_ERROR,
                    LogEvents::SetupWalletsForBtcPayStoreError.to_string()))
    }

    // TODO(AE: Log this securely.
    let _on_chain_payment_method_data_with_sensitive_data =
        on_chain_payment_method_data_with_sensitive_data.unwrap();


    log::info!("{:?}",
                LogEvents::SetupWalletsForBtcPayStoreOk);

    Ok(())
}

#[derive(Debug)]
enum LogEvents {
    SetupWalletsForBtcPayStoreStart,
    SetupWalletsForBtcPayStoreError,
    SetupWalletsForBtcPayStoreOk,
}

impl fmt::Display for LogEvents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogEvents::SetupWalletsForBtcPayStoreStart
                => write!(f, "SetupWalletsForBtcPayStoreStart"),
            LogEvents::SetupWalletsForBtcPayStoreError
                => write!(f, "SetupWalletsForBtcPayStoreError"),
            LogEvents::SetupWalletsForBtcPayStoreOk
                => write!(f, "SetupWalletsForBtcPayStoreOk"),
        }
    }
}

