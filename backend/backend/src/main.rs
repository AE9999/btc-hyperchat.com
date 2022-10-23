#[macro_use]
extern crate diesel;

mod client;
mod config;
mod web;
mod data;
mod app_context;
use actix_cors::Cors;
use actix_web::{App, HttpServer};
use std::str::FromStr;
use actix::Actor;
use log::LevelFilter;
use simple_logger::SimpleLogger;
use crate::client::clients::Clients;
use crate::config::config::config_from_cmd;
use crate::app_context::AppContext;
use crate::data::repos::Repositories;
use actix_web_httpauth::middleware::HttpAuthentication;
use actix_web_middleware_keycloak_auth::KeycloakAuth;

/*use paperclip::actix::{
    // extension trait for actix_web::App and proc-macro attributes
    OpenApiExt,
    // If you prefer the macro syntax for defining routes, import the paperclip macros
    // get, post, put, delete
    // use this instead of actix_web::web
    web::{self as activeWeb, resource, post, get},
};*/

use actix_web::web::{resource, post, get, patch, Data};


use crate::controller::btc_chat::register_btc_chat::register_btc_chat;
use web::model::invoice::callback::callback_mapping::CallbackMapping;
use web::controller::btc_chat::my_received_btc_chats::get_my_received_btc_chats::get_my_received_btc_chats;
use web::controller::btc_chat::my_received_btc_chats::process_btc_chat::process_btc_chat;
use web::controller::btc_chat::my_received_btc_chats::my_received_btc_chat_stream::stream_received_btc_chats;
use crate::controller::invoice::callback::invoice_received_payment::handle_invoice_received_payment_callback;
use web::controller::profile::get_profile::get_profile;
use crate::web::controller;
use crate::web::controller::user::create_user;
use web::model::btc_chat::my_received_btc_chat_stream::my_received_btc_chat_streamer::MyReceivedBtcChatStreamer;
use web::controller::btc_chat::my_received_btc_chats::get_test_btc_chat::get_test_btc_chat;
use crate::controller::profile::find_profiles::find_profiles;
use crate::controller::profile::top_profiles::top_profiles;
use web::controller::settings::webhook::test_store_webhook::test_store_webhook;
use web::controller::settings::webhook::store_webhook_activation::update_store_webhook_activation::update_store_webhook_activation;
use web::controller::settings::webhook::store_webhook_activation::get_store_webhook_activation::get_store_webhook_activation;
use web::controller::settings::webhook::store_webhook_config::get_store_webhook_config::get_store_webhook_config;
use web::controller::settings::webhook::store_webhook_config::update_store_webhook_config::update_store_webhook_config;
use crate::controller::settings::btc_pay::get_btc_pay_config::get_btc_pay_config;
use crate::controller::settings::btc_pay::reset_btc_pay_password::reset_btc_pay_password;
use crate::controller::settings::delete_account::delete_account;
use crate::web::service::util::get_decoding_key::get_decoding_key;


// Deal with https://auth0.com/blog/build-an-api-in-rust-with-jwt-authentication-using-actix-web/

// https://docs.rs/actix-web-httpauth/0.3.0-alpha.1/actix_web_httpauth/extractors/basic/struct.BasicAuth.html

// https://github.com/actix/actix-extras/blob/master/actix-web-httpauth/examples/middleware.rs

#[derive(Debug)]
enum LogEvents {
    ServerStarting
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {

    let context = Data::new(app_context::app_context());

    let host = context.config.server_config.host.clone();

    let port = context.config.server_config.port;

    let keycloak_pk =
        get_decoding_key(&context.config.keycloak_config);

    let mut keycloak_auth =
        KeycloakAuth::default_with_pk(keycloak_pk);
    keycloak_auth.detailed_responses = true;

    SimpleLogger::new()
                 .with_utc_timestamps()
                 .with_level(LevelFilter::from_str(context.config.general_config.debug_level.as_str()).unwrap())
                 .init()
                 .unwrap();

    log::info!("{:?} {:?}",
               LogEvents::ServerStarting,
               context.config);

    let btc_chat_stream =
        MyReceivedBtcChatStreamer::new(&context.config.keycloak_config).start();


    HttpServer::new(move || {
        let cors =
            Cors::default()
                .allowed_origin(context.config
                                       .server_config
                                      .cors_allowed_origin.as_str())
                .allowed_methods(vec!["GET", "POST", "OPTIONS", "PATCH"])
                .allow_any_header()
                .expose_any_header()
                .supports_credentials()
            ;

        App::new()
            // .wrap_api()
            .wrap(cors)
            .app_data(context.clone())
            .app_data(Data::new(btc_chat_stream.clone()))

            .service(resource("/api/user").route(post().to(create_user))
                                                   .wrap(HttpAuthentication::basic(crate::web::security::basic_auth_validator)))

            .service(resource("/api/profile/get/{username}").route(get().to(get_profile)))
            .service(resource("/api/profile/find/{username_prefix}").route(get().to(find_profiles)))
            .service(resource("/api/profile/top").route(get().to(top_profiles)))

            .service(resource("/api/btcchat/registerbtcchat").route(post().to(register_btc_chat)))

            .service(resource("/api/btcchat/myreceivedbtcchats")
                                .route(get().to(get_my_received_btc_chats))
                                .wrap(keycloak_auth.clone()))
            .service(resource("/api/btcchat/processbtcchat")
                                .route(post().to(process_btc_chat))
                                .wrap(keycloak_auth.clone()))
            .service(resource("/ws/btcchat/myreceivedbtcchatstream")
                                .route(get().to(stream_received_btc_chats)))
            .service(resource("/api/btcchat/testbtcchat")
                                .route(get().to(get_test_btc_chat))
                                .wrap(keycloak_auth.clone()))

            .service(resource("/api/settings/storewebhookactivation")
                                .route(get().to(get_store_webhook_activation))
                                .route(patch().to(update_store_webhook_activation))
                                .wrap(keycloak_auth.clone()))
            .service(resource("/api/settings/storewebhookconfig")
                .route(get().to(get_store_webhook_config))
                .route(patch().to(update_store_webhook_config))
                .wrap(keycloak_auth.clone()))
            .service(resource("/api/settings/teststorewebhook")
                .route(post().to(test_store_webhook))
                .wrap(keycloak_auth.clone()))
            .service(resource("/api/settings/btcpayconfig")
                .route(get().to(get_btc_pay_config))
                .wrap(keycloak_auth.clone()))
            .service(resource("/api/settings/resetbtcpaypassword")
                .route(post().to(reset_btc_pay_password))
                .wrap(keycloak_auth.clone()))
            .service(resource("/api/settings/deleteaccount")
                .route(post().to(delete_account))
                .wrap(keycloak_auth.clone()))

            .service(resource(CallbackMapping::ReceivedPayment.path())
                                .route(post().to(handle_invoice_received_payment_callback)))

    })
        .bind((host,
                     port))?
        .run()
        .await
}
