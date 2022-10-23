use std::fs;
use std::fs::File;
use std::io::Read;
use std::fmt;
use actix_web_middleware_keycloak_auth::DecodingKey;
use crate::config::keycloak::KeycloakConfig;


pub fn get_decoding_key(keycloak_config: &KeycloakConfig) -> DecodingKey {
    log::info!("{:?} {:?}",
                LogEvents::GetDecodingKeyStart,
                keycloak_config);
    let mut f = File::open(&keycloak_config.public_key_file)
                          .expect(LogEvents::GetDecodingKeyPublicKeyFileDoesNotExists.to_string().as_str());
    let metadata = fs::metadata(&keycloak_config.public_key_file)
                               .expect(LogEvents::GetDecodingKeyMetaDataUnreadable.to_string().as_str());
    let mut keycloak_pk = vec![0; metadata.len() as usize];
    f.read(&mut keycloak_pk)
     .expect(LogEvents::GetDecodingKeyBufferOverflow.to_string().as_str());
    let decoding_key = DecodingKey::from_rsa_pem(keycloak_pk.as_slice()).unwrap();

    log::info!("{:?}",
                LogEvents::GetDecodingKeyOk);

    decoding_key
}

#[derive(Debug)]
enum LogEvents {
    GetDecodingKeyStart,
    GetDecodingKeyPublicKeyFileDoesNotExists,
    GetDecodingKeyMetaDataUnreadable,
    GetDecodingKeyBufferOverflow,
    GetDecodingKeyOk
}

impl fmt::Display for LogEvents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogEvents::GetDecodingKeyStart
                => write!(f, "GetDecodingKeyStart"),
            LogEvents::GetDecodingKeyPublicKeyFileDoesNotExists
                => write!(f, "GetDecodingKeyPublicKeyFileDoesNotExists"),
            LogEvents::GetDecodingKeyMetaDataUnreadable
                => write!(f, "GetDecodingKeyMetaDataUnreadable"),
            LogEvents::GetDecodingKeyBufferOverflow
                => write!(f, "GetDecodingKeyBufferOverflow"),
            LogEvents::GetDecodingKeyOk
                => write!(f, "GetDecodingKeyOk"),
        }
    }
}
