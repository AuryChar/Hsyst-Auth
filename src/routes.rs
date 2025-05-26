#![allow(unused_imports)]
use actix_web::{App, HttpResponse, Responder, get, post, web};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
