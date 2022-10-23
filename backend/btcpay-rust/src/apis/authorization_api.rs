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


/// struct for typed errors of method [`api_keys_authorize`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiKeysAuthorizeError {
    UnknownValue(serde_json::Value),
}


/// Redirect the browser to this endpoint to request the user to generate an api-key with specific permissions
pub async fn api_keys_authorize(configuration: &configuration::Configuration, permissions: Option<Vec<String>>, application_name: Option<&str>, strict: Option<bool>, selective_stores: Option<bool>, redirect: Option<&str>, application_identifier: Option<&str>) -> Result<(), Error<ApiKeysAuthorizeError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api-keys/authorize", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = permissions {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("permissions".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("permissions", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = application_name {
        local_var_req_builder = local_var_req_builder.query(&[("applicationName", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = strict {
        local_var_req_builder = local_var_req_builder.query(&[("strict", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = selective_stores {
        local_var_req_builder = local_var_req_builder.query(&[("selectiveStores", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = redirect {
        local_var_req_builder = local_var_req_builder.query(&[("redirect", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = application_identifier {
        local_var_req_builder = local_var_req_builder.query(&[("applicationIdentifier", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<ApiKeysAuthorizeError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}
