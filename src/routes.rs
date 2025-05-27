#![allow(unused_imports)] // I use it to don't show unused imports, but you can remove it
// libraries
use actix_web::{App, HttpResponse, Responder, get, post, web};
use serde::{Deserialize, Serialize};

// routes
#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!") // hello world lol
}
