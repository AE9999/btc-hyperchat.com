/*
 * BTCPay Greenfield API
 *
 * A full API to use your BTCPay Server
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`greenfield_store_automated_lightning_payout_processors_controller_get_store_lightning_automated_payout_processors_for_payment_method`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GreenfieldStoreAutomatedLightningPayoutProcessorsControllerGetStoreLightningAutomatedPayoutProcessorsForPaymentMethodError {
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`greenfield_store_automated_lightning_payout_processors_controller_get_store_lightning_automated_transfer_sender_factory`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GreenfieldStoreAutomatedLightningPayoutProcessorsControllerGetStoreLightningAutomatedTransferSenderFactoryError {
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`greenfield_store_automated_lightning_payout_processors_controller_update_store_lightning_automated_payout_processor`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GreenfieldStoreAutomatedLightningPayoutProcessorsControllerUpdateStoreLightningAutomatedPayoutProcessorError {
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`greenfield_store_automated_on_chain_payout_processors_controller_get_store_on_chain_automated_payout_processors_for_payment_method`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GreenfieldStoreAutomatedOnChainPayoutProcessorsControllerGetStoreOnChainAutomatedPayoutProcessorsForPaymentMethodError {
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`greenfield_store_automated_on_chain_payout_processors_controller_get_store_on_chain_automated_transfer_sender_factory`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GreenfieldStoreAutomatedOnChainPayoutProcessorsControllerGetStoreOnChainAutomatedTransferSenderFactoryError {
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`greenfield_store_automated_on_chain_payout_processors_controller_update_store_on_chain_automated_payout_processor_for_payment_method`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GreenfieldStoreAutomatedOnChainPayoutProcessorsControllerUpdateStoreOnChainAutomatedPayoutProcessorForPaymentMethodError {
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`greenfield_store_automated_on_chain_payout_processors_controller_update_store_on_chain_automated_transfer_sender_factory`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GreenfieldStoreAutomatedOnChainPayoutProcessorsControllerUpdateStoreOnChainAutomatedTransferSenderFactoryError {
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`store_payout_processors_get_store_payout_processors`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StorePayoutProcessorsGetStorePayoutProcessorsError {
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`store_payout_processors_remove_store_payout_processor`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StorePayoutProcessorsRemoveStorePayoutProcessorError {
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}


/// Get configured store Lightning automated payout processors
pub async fn greenfield_store_automated_lightning_payout_processors_controller_get_store_lightning_automated_payout_processors_for_payment_method(configuration: &configuration::Configuration, store_id: &str, payment_method: &str) -> Result<Vec<crate::models::LightningAutomatedTransferSettings>, Error<GreenfieldStoreAutomatedLightningPayoutProcessorsControllerGetStoreLightningAutomatedPayoutProcessorsForPaymentMethodError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/stores/{storeId}/payout-processors/LightningAutomatedTransferSenderFactory/{paymentMethod}", local_var_configuration.base_path, storeId=crate::apis::urlencode(store_id), paymentMethod=crate::apis::urlencode(payment_method));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GreenfieldStoreAutomatedLightningPayoutProcessorsControllerGetStoreLightningAutomatedPayoutProcessorsForPaymentMethodError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get configured store Lightning automated payout processors
pub async fn greenfield_store_automated_lightning_payout_processors_controller_get_store_lightning_automated_transfer_sender_factory(configuration: &configuration::Configuration, store_id: &str) -> Result<Vec<crate::models::LightningAutomatedTransferSettings>, Error<GreenfieldStoreAutomatedLightningPayoutProcessorsControllerGetStoreLightningAutomatedTransferSenderFactoryError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/stores/{storeId}/payout-processors/LightningAutomatedTransferSenderFactory", local_var_configuration.base_path, storeId=crate::apis::urlencode(store_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GreenfieldStoreAutomatedLightningPayoutProcessorsControllerGetStoreLightningAutomatedTransferSenderFactoryError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update configured store Lightning automated payout processors
pub async fn greenfield_store_automated_lightning_payout_processors_controller_update_store_lightning_automated_payout_processor(configuration: &configuration::Configuration, store_id: &str, payment_method: &str, update_lightning_automated_transfer_settings: crate::models::UpdateLightningAutomatedTransferSettings) -> Result<crate::models::LightningAutomatedTransferSettings, Error<GreenfieldStoreAutomatedLightningPayoutProcessorsControllerUpdateStoreLightningAutomatedPayoutProcessorError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/stores/{storeId}/payout-processors/LightningAutomatedTransferSenderFactory/{paymentMethod}", local_var_configuration.base_path, storeId=crate::apis::urlencode(store_id), paymentMethod=crate::apis::urlencode(payment_method));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&update_lightning_automated_transfer_settings);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GreenfieldStoreAutomatedLightningPayoutProcessorsControllerUpdateStoreLightningAutomatedPayoutProcessorError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get configured store onchain automated payout processors
pub async fn greenfield_store_automated_on_chain_payout_processors_controller_get_store_on_chain_automated_payout_processors_for_payment_method(configuration: &configuration::Configuration, store_id: &str, payment_method: &str) -> Result<Vec<crate::models::OnChainAutomatedTransferSettings>, Error<GreenfieldStoreAutomatedOnChainPayoutProcessorsControllerGetStoreOnChainAutomatedPayoutProcessorsForPaymentMethodError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/stores/{storeId}/payout-processors/OnChainAutomatedTransferSenderFactory/{paymentMethod}", local_var_configuration.base_path, storeId=crate::apis::urlencode(store_id), paymentMethod=crate::apis::urlencode(payment_method));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GreenfieldStoreAutomatedOnChainPayoutProcessorsControllerGetStoreOnChainAutomatedPayoutProcessorsForPaymentMethodError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get configured store onchain automated payout processors
pub async fn greenfield_store_automated_on_chain_payout_processors_controller_get_store_on_chain_automated_transfer_sender_factory(configuration: &configuration::Configuration, store_id: &str) -> Result<Vec<crate::models::OnChainAutomatedTransferSettings>, Error<GreenfieldStoreAutomatedOnChainPayoutProcessorsControllerGetStoreOnChainAutomatedTransferSenderFactoryError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/stores/{storeId}/payout-processors/OnChainAutomatedTransferSenderFactory", local_var_configuration.base_path, storeId=crate::apis::urlencode(store_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GreenfieldStoreAutomatedOnChainPayoutProcessorsControllerGetStoreOnChainAutomatedTransferSenderFactoryError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update configured store onchain automated payout processors
pub async fn greenfield_store_automated_on_chain_payout_processors_controller_update_store_on_chain_automated_payout_processor_for_payment_method(configuration: &configuration::Configuration, store_id: &str, payment_method: &str, update_on_chain_automated_transfer_settings: crate::models::UpdateOnChainAutomatedTransferSettings) -> Result<crate::models::OnChainAutomatedTransferSettings, Error<GreenfieldStoreAutomatedOnChainPayoutProcessorsControllerUpdateStoreOnChainAutomatedPayoutProcessorForPaymentMethodError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/stores/{storeId}/payout-processors/OnChainAutomatedTransferSenderFactory/{paymentMethod}", local_var_configuration.base_path, storeId=crate::apis::urlencode(store_id), paymentMethod=crate::apis::urlencode(payment_method));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&update_on_chain_automated_transfer_settings);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GreenfieldStoreAutomatedOnChainPayoutProcessorsControllerUpdateStoreOnChainAutomatedPayoutProcessorForPaymentMethodError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update configured store onchain automated payout processors
pub async fn greenfield_store_automated_on_chain_payout_processors_controller_update_store_on_chain_automated_transfer_sender_factory(configuration: &configuration::Configuration, store_id: &str, update_on_chain_automated_transfer_settings: crate::models::UpdateOnChainAutomatedTransferSettings) -> Result<crate::models::OnChainAutomatedTransferSettings, Error<GreenfieldStoreAutomatedOnChainPayoutProcessorsControllerUpdateStoreOnChainAutomatedTransferSenderFactoryError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/stores/{storeId}/payout-processors/OnChainAutomatedTransferSenderFactory", local_var_configuration.base_path, storeId=crate::apis::urlencode(store_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&update_on_chain_automated_transfer_settings);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GreenfieldStoreAutomatedOnChainPayoutProcessorsControllerUpdateStoreOnChainAutomatedTransferSenderFactoryError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get store configured payout processors
pub async fn store_payout_processors_get_store_payout_processors(configuration: &configuration::Configuration, store_id: &str) -> Result<Vec<crate::models::PayoutProcessorData>, Error<StorePayoutProcessorsGetStorePayoutProcessorsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/stores/{storeId}/payout-processors", local_var_configuration.base_path, storeId=crate::apis::urlencode(store_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<StorePayoutProcessorsGetStorePayoutProcessorsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Remove store configured payout processor
pub async fn store_payout_processors_remove_store_payout_processor(configuration: &configuration::Configuration, store_id: &str, processor: &str, payment_method: &str) -> Result<(), Error<StorePayoutProcessorsRemoveStorePayoutProcessorError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/stores/{storeId}/payout-processors/{processor}/{paymentMethod}", local_var_configuration.base_path, storeId=crate::apis::urlencode(store_id), processor=crate::apis::urlencode(processor), paymentMethod=crate::apis::urlencode(payment_method));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<StorePayoutProcessorsRemoveStorePayoutProcessorError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}
