#[allow(unused)]

use std::net::SocketAddr;

use axum::{Router, extract::{Path, Query}, response::{Html, IntoResponse}, routing::get};
use serde::Deserialize;

#[tokio::main]
async fn main(){
    let hello_routes = Router::new()
    .route("/hello",get(handler_hello))
    .route("/hello2/{name}", get(handler_hello2));

    // #region --- Server init ---
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, hello_routes).await.unwrap();
    // #endregion --- Server init ---
}
#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>
}

// #region --- Handler Hello ---
// e.g., `/hello?name=Danton`
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("<h1>Hello {name}</h1>"))
}
// #endregion --- Handler Hello ---

// #region --- Handler hello 2 ---
// e.g., `/hello2/Danton/`
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello2 - {name}", "HANDLER_HELLO2");
    Html(format!("<h2>Hello {name}</h2>"))
}
// #endregion --- Handler hello 2 ---