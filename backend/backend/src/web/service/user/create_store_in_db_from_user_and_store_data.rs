use std::collections::HashMap;
use actix_web::http::StatusCode;
use btcpay_rust::models::StoreData;
use crate::AppContext;
use crate::data::model::store::Store;
use crate::data::model::user::User;
use std::fmt;
use actix_web::web::Data;
use crate::web::model::settings::store_webhook_config::post_type::PostType;
use crate::web::model::settings::store_webhook_config::store_webhook_config::StoreWebhookConfig;
use crate::web::service::profile::serialize_store_webhook_config::serialize_store_webhook_config;

pub fn create_store_in_db_from_user_and_store_data(user: &User,
                                                   store_data: &StoreData,
                                                   context: &Data<AppContext>) -> Result<Store,
                                                                                      (StatusCode, String)> {

    log::info!("{:?}, {:?}, {:?}",
                LogEvents::CreateStoreInDbFromUserAndStoreDataStart,
                user,
                store_data);

    let default_webhook_config =
        create_default_streamlabs_store_webhook();

    let webhook_config_json =
        serialize_store_webhook_config(default_webhook_config)
            .expect(LogEvents::CreateStoreInDbFromUserAndStoreDataError.to_string().as_str());

    let store =
        context.repositories.store_repository.create(user.id.clone(),
                                                     store_data.id.clone().unwrap(),
                                                     webhook_config_json,
                                                     false,
                                                     false,
                                                     true);
    if store.is_err() {
        log::error!("{:?}, {:?}",
                     LogEvents::CreateStoreInDbFromUserAndStoreDataError,
                     store.as_ref().err().unwrap().to_string());
        return Err((StatusCode::INTERNAL_SERVER_ERROR,
                    LogEvents::CreateStoreInDbFromUserAndStoreDataError.to_string()));
    }

    log::info!("{:?}",
                LogEvents::CreateStoreInDbFromUserAndStoreDataOk);

    Ok(store.unwrap())
}

fn create_default_streamlabs_store_webhook() -> StoreWebhookConfig {
    /*
    curl --request POST \
     --url 'https://streamlabs.com/api/v1.0/donations' \
     -d "name=name&identifier=identifier&amount=amount&currency=currency&access_token=access_token"
     */
    let url = "https://streamlabs.com/api/v1.0/donations".to_string();

    let mut body_attributes: HashMap<String, String> = HashMap::new();
    body_attributes.insert("name".to_string(), "{{sender}}".to_string());
    body_attributes.insert("identifier".to_string(), "{{sender}}".to_string());
    body_attributes.insert("amount".to_string(), "{{amount_in_fiat}}".to_string());
    body_attributes.insert("currency".to_string(), "{{currency}}".to_string());
    body_attributes.insert("message".to_string(), "{{message}}".to_string());
    body_attributes.insert("access_token".to_string(), "TO_BE_FILLED_IN_BY_USER".to_string());
    let query_attributes: HashMap<String, String> = HashMap::new();
    let header_attributes : HashMap<String, String>  = HashMap::new();
    let post_type: PostType = PostType::ApplicationXWwwFormUrlEncoded;

    StoreWebhookConfig::new(Some(url),
                            Some(body_attributes),
                            Some(query_attributes),
                            Some(header_attributes),
                            Some(post_type))
}

#[derive(Debug)]
enum LogEvents {
    CreateStoreInDbFromUserAndStoreDataStart,
    CreateStoreInDbFromUserAndStoreDataError,
    CreateStoreInDbFromUserAndStoreDataOk,
}

impl fmt::Display for LogEvents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogEvents::CreateStoreInDbFromUserAndStoreDataStart => write!(f, "CreateStoreInDbFromUserAndStoreDataStart"),
            LogEvents::CreateStoreInDbFromUserAndStoreDataError => write!(f, "CreateStoreInDbFromUserAndStoreDataError"),
            LogEvents::CreateStoreInDbFromUserAndStoreDataOk => write!(f, "CreateStoreInDbFromUserAndStoreDataOk"),
        }
    }
}
