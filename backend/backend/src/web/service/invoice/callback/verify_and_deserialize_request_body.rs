use std::fmt;
use actix_web::http::StatusCode;
use actix_web::HttpRequest;
use actix_web::web::{BytesMut, Data};
use crate::AppContext;
use futures_util::stream::StreamExt;
use serde::de::DeserializeOwned;
use actix_web::web::Payload;

// https://docs.rs/hmac/0.12.1/hmac/

// https://stackoverflow.com/questions/68170643/how-to-get-body-of-request-as-text-with-actix

use hmac::{Hmac, Mac};

type HmacSha256 = Hmac<sha2::Sha256>;

pub async fn verify_and_deserialize_request_body<T: DeserializeOwned + std::fmt::Debug>(req: &HttpRequest,
                                                                                        body: &mut Payload,
                                                                                        context: &Data<AppContext>) -> Result<T,
                                                            (StatusCode, String)>  {

    log::info!("{:?}, {:?}",
               LogEvents::VerifyAndDeserializeRequestBodyStart,
               req);

    let mut bytes = BytesMut::new();
    while let Some(item) = body.next().await {
        if item.is_err() {
            log::info!("{:?}",
                   LogEvents::VerifyAndDeserializeRequestBodyPayloadError);
            return Err((StatusCode::BAD_REQUEST,
                        LogEvents::VerifyAndDeserializeRequestBodyPayloadError
                            .to_string()));
        }

        let item = item.unwrap();

        bytes.extend_from_slice(&item);
    }

    let debug =  String::from_utf8(bytes.to_vec()).unwrap();
    log::info!("{:?}", debug);

    let header = req.headers()
                                       .get("btcpay-sig");

    if header.is_none() {
        log::info!("{:?}",
                   LogEvents::VerifyAndDeserializeRequestBodyHeaderMissingError);
        return Err((StatusCode::FORBIDDEN,
                    LogEvents::VerifyAndDeserializeRequestBodyHeaderMissingError
                              .to_string()));
    }

    let header = header.unwrap();

    let raw_key = context.config.btc_pay_config
                                .callback_secret
                                .clone();
    let key = raw_key.as_bytes();
    let mut mac =
        HmacSha256::new_from_slice(key.clone())
                   .expect("HMAC can take key of any size");

    mac.update(bytes.as_ref());

    let actual_key_raw = mac.finalize().into_bytes();

    let actual = format!("sha256={}", hex::encode(actual_key_raw.as_slice()));

    let expected = header.to_str().unwrap_or_default().to_string();

    if !actual.eq(&expected) {
        log::info!("{:?}, {:?}, {:?}",
                   LogEvents::VerifyAndDeserializeRequestBodyHeaderWrongValue,
                   actual,
                   expected);
        return Err((StatusCode::FORBIDDEN,
                    LogEvents::VerifyAndDeserializeRequestBodyHeaderWrongValue
                              .to_string()));
    }

    let object : serde_json::Result<T> = serde_json::from_slice(bytes.to_vec().as_slice());

    if object.is_err() {
        log::info!("{:?}, {:?}",
                   LogEvents::VerifyAndDeserializeRequestBodyDeserializationError,
                   object.err().unwrap());
        return Err((StatusCode::FORBIDDEN,
                    LogEvents::VerifyAndDeserializeRequestBodyDeserializationError
                        .to_string()));
    }

    let object = object.unwrap();

    log::info!("{:?}",
               LogEvents::VerifyAndDeserializeRequestBodyOk);

    Ok(object)
}

#[derive(Debug)]
enum LogEvents {
    VerifyAndDeserializeRequestBodyStart,
    VerifyAndDeserializeRequestBodyPayloadError,
    VerifyAndDeserializeRequestBodyDeserializationError,
    VerifyAndDeserializeRequestBodyHeaderMissingError,
    VerifyAndDeserializeRequestBodyHeaderWrongValue,
    VerifyAndDeserializeRequestBodyOk
}

impl fmt::Display for LogEvents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogEvents::VerifyAndDeserializeRequestBodyStart
                => write!(f, "VerifyAndDeserializeRequestBodyStart"),
            LogEvents::VerifyAndDeserializeRequestBodyPayloadError
                => write!(f, "VerifyAndDeserializeRequestBodyPayloadError"),
            LogEvents::VerifyAndDeserializeRequestBodyDeserializationError
                => write!(f, "VerifyAndDeserializeRequestBodyDeserializationError"),
            LogEvents::VerifyAndDeserializeRequestBodyHeaderMissingError
                => write!(f, "VerifyAndDeserializeRequestBodyHeaderMissingError"),
            LogEvents::VerifyAndDeserializeRequestBodyHeaderWrongValue
                => write!(f, "VerifyAndDeserializeRequestBodyHeaderWrongValue"),
            LogEvents::VerifyAndDeserializeRequestBodyOk
                => write!(f, "VerifyAndDeserializeRequestBodyOk"),
        }
    }
}
