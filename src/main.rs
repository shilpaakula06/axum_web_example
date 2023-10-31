use std::net::SocketAddr;
use axum::{handler::get, Router};

async fn hello() -> String {
    "Hello, Axum!".to_string()
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

