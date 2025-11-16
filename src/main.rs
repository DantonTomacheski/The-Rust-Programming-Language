use std::net::SocketAddr;

use axum::{Router, response::Html, routing::get};

#[tokio::main]
async fn main() {
    let hello_routes = Router::new().route(
        "/hello-world",
        get(|| async { Html("<h1>Hello Jovana</h1>") })
    );

    let addr = SocketAddr::from(([127,0,0,1], 8080));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, hello_routes).await.unwrap();
}