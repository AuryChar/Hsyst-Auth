#![allow(unused_imports)] // I use it to don't show unused imports, but you can remove it
// libraries
use actix_web::{App, HttpResponse, Responder, get, post, web};
use serde::{Deserialize, Serialize};
use crate::middlewares;
use rusqlite::Connection;
use std::sync::Arc;
use std::sync::Mutex;

// routes
#[get("/")]
pub async fn index(db: web::Data<Arc<Mutex<Connection>>>) -> impl Responder { // DEBUG: test db
    let conn = db.lock().unwrap();
    conn.execute("CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY, email TEXT, password TEXT)", []).unwrap();
    let email = "johndoe@example";
    let password = "123";
    conn.execute("INSERT INTO users (email, password) VALUES (?, ?)", &[&email, &password]).unwrap();
    HttpResponse::Ok().body(middlewares::generate_token(email.to_string(), password.to_string())) // DEBUG: test token
}
