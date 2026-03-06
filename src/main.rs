// src/main.rs
use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(handler));

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Server listening on {}", addr);
    axum::serve(tokio::net::TcpListener::bind(&addr).await.unwrap(), app)
        .await
        .unwrap();
}

async fn handler() -> &'static str {
    "Hello, Rust HTTP Server!"
}