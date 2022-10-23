use std::collections::HashMap;
use crate::data::model::btc_chat::BtcChat;
use std::fmt;
use actix_web::http::StatusCode;
use std::time::Duration;
use crate::web::model::invoice::callback::webhook::post_details::PostDetails;
use actix_web::web::Data;
use crate::AppContext;
use handlebars::Handlebars;
use crate::web::model::settings::store_webhook_config::post_type::PostType;
use crate::web::model::settings::store_webhook_config::store_webhook_config::StoreWebhookConfig;


pub async fn make_callback_per_store_webhook_config(store_webhook_config: &StoreWebhookConfig,
                                                    btc_chat: &BtcChat,
                                                    context: &Data<AppContext>) -> Result<(),
                                                                         (StatusCode, String)> {

    log::info!("{:?}, {:?}, {:?}",
               LogEvents::MakeCallbackPerStoreWebhookConfigStart,
               store_webhook_config,
               btc_chat);

    if !store_webhook_config.has_minimally_needed_attributes() {
        log::error!("{:?}, {:?}",
                    LogEvents::MakeCallbackPerStoreWebhookConfigIncompleteConfigError,
                    store_webhook_config);
        return Err((StatusCode::INTERNAL_SERVER_ERROR,
                    LogEvents::MakeCallbackPerStoreWebhookConfigIncompleteConfigError.to_string()))
    }


    let post_details =
        get_post_details(store_webhook_config,
                         btc_chat)?;

    perform_post(post_details,
                 context).await?;

    log::info!("{:?}",
               LogEvents::MakeCallbackPerStoreWebhookConfigOk);
    Ok(())
}

fn get_post_details(store_webhook_config: &StoreWebhookConfig,
                    btc_chat: &BtcChat) -> Result<PostDetails, (StatusCode, String)> {

    let default_error =
        LogEvents::MakeCallbackPerStoreWebhookConfigIncompleteConfigError
                  .to_string();

    let post_type =
        PostType::from_code(store_webhook_config.post_type_code
                                                     .expect(default_error.as_str()));

    if post_type.is_undefined() {
        log::error!("{:?}, {:?}",
                    LogEvents::MakeCallbackPerStoreWebhookConfigIncompleteConfigError,
                    store_webhook_config);
        return Err((StatusCode::INTERNAL_SERVER_ERROR,
                    default_error))
    }

    let url : String =
        store_webhook_config.url
                            .as_ref()
                            .expect(default_error.as_str())
                            .clone();

    let mut header_elements: HashMap<String, String> = HashMap::new();
    fill_values(&mut header_elements,
                &store_webhook_config.header_attributes,
                btc_chat)?;

    let mut body: HashMap<String, String> = HashMap::new();
    fill_values(&mut body,
                &store_webhook_config.data_attributes,
                btc_chat)?;

    let mut query_elements: HashMap<String, String> = HashMap::new();
    fill_values(&mut query_elements,
                &store_webhook_config.query_attributes,
                btc_chat)?;

    let post_details =
        PostDetails::new(url,
                         post_type,
                         header_elements,
                         body,
                         query_elements);



    Ok(post_details)
}

fn fill_values(map: &mut HashMap<String,String>,
               values: &Option<HashMap<String, String>>,
               btc_chat: &BtcChat) -> Result<(), (StatusCode, String)> {
    if values.is_none() {
        return Ok(());
    }

    let values = values.as_ref().unwrap().clone(); // Yes inefficient. Sue me.


    for value in values {

        log::info!("Input -> {:?}, {:?}",
                   value.0,
                   value.1);

        let interpolated_value =
            interpolate_template(value.1,
                                 btc_chat)?;

        log::info!("Output -> {:?}", interpolated_value);

        map.insert(value.0, interpolated_value);

    }

    Ok(())
}

async fn perform_post(post_details: PostDetails,
                context: &Data<AppContext>) -> Result<(), (StatusCode, String)> {

    let mut post =
        reqwest::Client::new()
            .post(post_details.url)
            .timeout(Duration::from_secs(context.config.server_config
                                                     .callback_to_user_timeout));

    post =
        if post_details.post_type.is_json() {
            post.json(&post_details.body)
        } else {
            post.form(&post_details.body)
        };

    for header in post_details.header_elements {
        post = post.header(header.0, header.1);
    }

    post = post.query(&post_details.query_elements);

    let ok = post.send().await;

    if ok.is_err() {
        // Should be caught at a higher level
        log::warn!("{:?}, {:?}",
                    LogEvents::MakeCallbackPerStoreWebhookConfigPostError,
                    ok.as_ref()
                      .err()
                      .expect(LogEvents::MakeCallbackPerStoreWebhookConfigPostError.to_string()
                                                                           .as_str()));
        return Err((StatusCode::INTERNAL_SERVER_ERROR,
                    LogEvents::MakeCallbackPerStoreWebhookConfigPostError.to_string()))
    }

    Ok(())
}


fn interpolate_template(template: String, btc_chat: &BtcChat) -> Result<String, (StatusCode, String)> {
    let reg = Handlebars::new();

    // let json = serde_json::to_string(btc_chat);
    //
    // if json.is_err() {
    //     log::warn!("{:?}, {:?}",
    //                 LogEvents::MakeCallbackPerStoreWebhookConfigRenderingError,
    //                 json.as_ref()
    //                   .err()
    //                   .expect(LogEvents::MakeCallbackPerStoreWebhookConfigRenderingError.to_string()
    //                                                                        .as_str()));
    //     return Err((StatusCode::INTERNAL_SERVER_ERROR,
    //                 LogEvents::MakeCallbackPerStoreWebhookConfigRenderingError.to_string()))
    // }
    //
    // let json = json.unwrap();

    let value = reg.render_template(template.as_str(),
                                    &btc_chat);

    if value.is_err() {
        log::warn!("{:?}, {:?}",
                    LogEvents::MakeCallbackPerStoreWebhookConfigRenderingError,
                    value.as_ref()
                      .err()
                      .expect(LogEvents::MakeCallbackPerStoreWebhookConfigRenderingError.to_string()
                                                                           .as_str()));
        return Err((StatusCode::INTERNAL_SERVER_ERROR,
                    LogEvents::MakeCallbackPerStoreWebhookConfigRenderingError.to_string()))
    }

    let value = value.unwrap();

    Ok(value)
}



#[derive(Debug)]
enum LogEvents {
    MakeCallbackPerStoreWebhookConfigStart,
    MakeCallbackPerStoreWebhookConfigIncompleteConfigError,
    MakeCallbackPerStoreWebhookConfigRenderingError,
    MakeCallbackPerStoreWebhookConfigPostError,
    MakeCallbackPerStoreWebhookConfigOk,
}

impl fmt::Display for LogEvents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogEvents::MakeCallbackPerStoreWebhookConfigStart
                => write!(f, "SendUpdateToUserStart"),
            LogEvents::MakeCallbackPerStoreWebhookConfigIncompleteConfigError
            => write!(f, "SendCallbackToUserUndefinedPostMethodError"),
            LogEvents::MakeCallbackPerStoreWebhookConfigRenderingError
                => write!(f, "SendCallbackToUserWebhookRenderingError"),
            LogEvents::MakeCallbackPerStoreWebhookConfigPostError
                => write!(f, "SendCallbackToUserWebhookPostError"),
            LogEvents::MakeCallbackPerStoreWebhookConfigOk
                => write!(f, "SendCallbackToUserWebhookOk"),

        }
    }
}
