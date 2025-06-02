use jsonwebtoken::{encode,decode, DecodingKey, Validation, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::env;
use time::{Duration, OffsetDateTime};

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn generate_token(id: String) -> String {
    let expiration = OffsetDateTime::now_utc() + Duration::days(1);
    let key = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let claims = Claims {
        sub: id,
        exp: expiration.unix_timestamp() as usize,
    };

    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(key.as_bytes())).unwrap();
    token
}

pub fn verify_token(token: String) -> Result<Claims, String> {
    let key = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let token_data = decode::<Claims>(&token, &DecodingKey::from_secret(key.as_bytes()), &Validation::default()).unwrap();
    Ok(token_data.claims)
}