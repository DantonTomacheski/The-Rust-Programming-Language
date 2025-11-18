#[allow(unused)]

use std::net::SocketAddr;

pub use self::error::{Error,Result};
use axum::{Json, Router, extract::{Path, Query}, response::{Html, IntoResponse}, routing::get};
use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;

mod error;
#[tokio::main]
async fn main(){
    let routes_all = Router::new()
     .merge(routes_hello())
     .fallback_service(routes_static());
    
    // region: --- Start Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, routes_all).await.unwrap();
    // endregion: --- Start Server
}

// region: Static routing
fn routes_static() -> ServeDir {
    println!("->> {:<12} - routes_static", "ROUTES_STATIC");
    ServeDir::new("./")
}
// endregion: Static routing

// region: --- Routes Hello
fn routes_hello() -> Router {
    Router::new()
        .route("/hello",get(handler_hello))
        .route("/hello2/{name}", get(handler_hello2))
        .route("/hello-json/{name}", get(handler_hello_json))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>
}

#[derive(Serialize)]
struct HelloResponse {
    message: String,
}

// e.g., `/hello?name=Danton`
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("<h1>Hello {name}</h1>"))
}

// e.g., `/hello2/Danton/`
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler - {name}", "HANDLER_HELLO2");
    Html(format!("<h2>Hello {name}</h2>"))
}

async fn handler_hello_json(Path(name): Path<String>) -> impl IntoResponse {
    let response = HelloResponse {
        message: format!("Hello {name}")
    };

    Json(response)
}
// endregion: --- Routes Hello