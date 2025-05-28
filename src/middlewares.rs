#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
use jsonwebtoken::{encode,decode, DecodingKey, Validation, EncodingKey, Header};
use actix_web::{http, web};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub sub: (String, String),
    pub exp: usize,
}


pub fn generate_token(email: String, password: String) -> String {
    let key = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let claims = Claims {
        sub: (email, password), // DEBUG: test claims
        exp: (chrono::Utc::now() + chrono::Duration::minutes(30)).timestamp() as usize
    };
    println!("Claims: {:#?}", claims); // DEBUG: test claims

    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(key.as_bytes())).unwrap();
    token
}