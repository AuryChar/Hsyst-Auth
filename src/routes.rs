#![allow(unused_imports)] // I use it to don't show unused imports, but you can remove it
// libraries
use actix_web::{App, HttpResponse, Responder, get, post, web, cookie::{Cookie, SameSite}};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::middlewares;
use rusqlite::Connection;
use std::sync::Arc;
use std::sync::Mutex;
use time::Duration;
use bcrypt::{hash, verify, DEFAULT_COST};

// cookies config
fn create_cookie(token: String) -> Cookie<'static> {
    let cookie = Cookie::build("token", token)
        .path("/")
        .secure(true)
        .same_site(SameSite::Strict)
        .http_only(true)
        .max_age(Duration::days(1))
        .finish();
    cookie
}

// structs
#[derive(Serialize, Deserialize)]
pub struct RegisterForm {
    email: String,
    password: String,
}

// routes
#[post("/register")]
pub async fn register(db: web::Data<Arc<Mutex<Connection>>>, req: web::Json<RegisterForm>) -> impl Responder {
    let conn = match db.lock() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": "Could not acquire database lock"
        })),
    };
    
    if let Err(e) = conn.execute(
        "CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY AUTOINCREMENT, email TEXT NOT NULL UNIQUE, password TEXT NOT NULL)", // TODO: add this in server init
        []
    ) {
        return HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": format!("Database error: {}", e)
        }));
    }

    let email = req.email.clone();
    let password = req.password.clone();
    
    let password_hash = match hash(password, DEFAULT_COST) {
        Ok(hash) => hash,
        Err(_) => return HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": "Could not hash password"
        })),
    };

    let email_exists: Result<i64, _> = conn.query_row(
        "SELECT COUNT(*) FROM users WHERE email = ?", 
        &[&email], 
        |row| row.get(0)
    );

    match email_exists {
        Ok(count) if count > 0 => {
            return HttpResponse::Conflict().json(json!({
                "status": "error",
                "message": "Email already exists"
            }));
        },
        Err(e) => {
            return HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": format!("Database error: {}", e)
            }));
        },
        _ => (),
    }

    if let Err(e) = conn.execute(
        "INSERT INTO users (email, password) VALUES (?, ?)", 
        [&email, &password_hash]
    ) {
        return HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": format!("Could not create user: {}", e)
        }));
    }

    HttpResponse::Ok().json(json!({
        "status": "success",
        "message": "User registered successfully"
    }))
}