#![warn(missing_docs)]
#![warn(clippy::all)]

//! Flexible Project server.

use actix_web::{get, App, HttpServer, Responder};

/// The most simple HTTP GET endpoint.
#[get("/")]
async fn hello() -> impl Responder {
    "Hello world!"
}

/// Entry point of the server.
#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
