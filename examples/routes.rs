use std::net::SocketAddr;

use axum::{Router, extract::{Query}, response::{Html, IntoResponse}, routing::get};
use serde::Deserialize;
use tower_http::services::ServeDir;

async fn routes() {
    let routes_all = Router::new()
        .merge(routes_hello())
        .fallback_service(routes_static());

    fn routes_static() -> ServeDir {
        println!("->> {:<12} - routes_static", "ROUTES_STATIC");
        ServeDir::new("./")
    }

    // region: Hello routes
    #[derive(Debug, Deserialize)]
    struct HelloParams {
        name: Option<String>,
    }

    fn routes_hello() -> Router {
        Router::new()
            .route("/hello", get(handler_query))
    }

    async fn handler_query(Query(params): Query<HelloParams>) -> impl IntoResponse {
        println!("{:<12} - handler", "HANDLER");
        let name = params.name.as_deref().unwrap_or("World");
        Html(format!("Hello <strong>{name:?}</strong>"))
    }

    
    // endregion: Hello routes

    // region: Start server
    let addr = SocketAddr::from(([127,0,0,1], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("listening on: {addr}");
    axum::serve(listener, routes_all).await.unwrap()
    // endregion: Start server
}

#[tokio::main]
async fn main() {
    routes().await;
}