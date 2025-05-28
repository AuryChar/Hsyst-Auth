/// # Main file
/// This file just runs the server

// seting up crates
extern crate bcrypt;
extern crate serde;

// libraries
use actix_web::{App, HttpServer};

// modules
pub mod middlewares;
pub mod routes;

// running the server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    println!("Server running at http://localhost:8080");
    HttpServer::new(|| {
        App::new()
            .service(routes::index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}