use std::time::{SystemTime, Duration, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use crate::prelude::*;
#[derive(Deserialize, Serialize)]
pub struct Claims {
    pub(crate) sub: String,
    pub(crate) exp: usize
}

pub fn decode(jwt: &str, secret: &str) -> Result<Claims> {
    let mut validation = jsonwebtoken::Validation::default();
    validation.validate_exp = true;
    let key = jsonwebtoken::DecodingKey::from_secret(secret.as_bytes());
    let claims = jsonwebtoken::decode::<Claims>(jwt, &key, &validation)?.claims;
    Ok(claims)
}

pub fn encode(uid: String, secret: &str) -> Result<String> {
    let exp = (SystemTime::now() + Duration::from_secs_f64(86400.0 * 7.0)).duration_since(UNIX_EPOCH).unwrap().as_secs();
    let claims = Claims {
        sub: uid,
        exp: exp as usize
    };
    let header = jsonwebtoken::Header::default();
    let key = jsonwebtoken::EncodingKey::from_secret(secret.as_bytes());
    let res = jsonwebtoken::encode(&header, &claims, &key)?;
    Ok(res)
}

pub fn verify_jwt(jwt: &str, secret: &str) -> Result<Claims> {
    let claims = decode(jwt, secret)?;
    Ok(claims)
}