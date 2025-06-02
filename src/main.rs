// seting up crates
extern crate bcrypt;
extern crate serde;

// libraries
use actix_web::{web, App, HttpServer};
use rusqlite::Connection;
use std::sync::Arc;
use std::sync::Mutex;
use actix_files as fs;

// modules
pub mod middlewares;
pub mod routes;

// running the server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    println!("Server running at http://localhost:8080");
    let conn = Arc::new(Mutex::new(Connection::open("database.db").unwrap()));
    match conn.lock() {
        Ok(_) => println!("Database connected"),    
        Err(_) => println!("Database not connected"),
    }
    HttpServer::new(move || {
        App::new()
            .service(routes::register)
            .service(routes::login)
            .service(routes::verify_user)
            .service(routes::logout)
            .service(fs::Files::new("/", "./static").index_file("index.html"))
            .app_data(web::Data::new(conn.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}