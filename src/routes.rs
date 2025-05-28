#![allow(unused_imports)] // I use it to don't show unused imports, but you can remove it
// libraries
use actix_web::{App, HttpResponse, Responder, get, post, web};
use serde::{Deserialize, Serialize};
use crate::middlewares;

// routes
#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body(middlewares::generate_token("test".to_string(), "123".to_string())) // DEBUG: test token
}
