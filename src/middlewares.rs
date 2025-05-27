#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
use jsonwebtoken::{encode,decode, DecodingKey, Validation, EncodingKey, Header};
use std::env;
use actix_web::{http, web};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn generate_token() -> String {
    let key = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let claims = Claims {
        sub: "test".to_string(), // TODO: change this after, it's just for testing
        exp: (chrono::Utc::now() + chrono::Duration::minutes(30)).timestamp() as usize
    };

    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(key.as_bytes())).unwrap();
    token
}