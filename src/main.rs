#![allow(unused)]

mod error;
mod web;

pub use self::error::{Error, Result};

use axum::{
    body::Bytes, extract::{Path, Query}, middleware, response::{Html, IntoResponse, Response}, routing::get, Router
};
use tower_cookies::CookieManagerLayer;
use std::{
    collections::HashMap,
    sync::{Arc,RwLock},
};
use serde::{Deserialize, Serialize};

// Creating a shared resource with an Arc Lock for
// multi-read and single-write access
type SharedState = Arc<RwLock<AppState>>;

// Shared In-memory key-value store for server 
#[derive(Default)]  
struct AppState {
    db: HashMap<String, Bytes>,
}

#[tokio::main]
async fn main(){
    let router = Router::new()
    .merge(routes_root())
    .merge(web::routes_login::routes())
    .layer(middleware::map_response(main_response_mapper))
    .layer(CookieManagerLayer::new());
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("->> LISTENINING ON localhost:8080\n");
    axum::serve(listener, router).await.unwrap();
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");

    println!();
    res
}

fn routes_root() -> Router{
    Router::new().route(
        "/hello",
        get(hello_world_handler)
    )
    .route(
        "/helloQuery",
        get(hello_world_query_handler)
    )
    .route(
        "/hello/:name",
        get(hello_world_path_handler)
    )
}

async fn hello_world_handler() -> impl IntoResponse{
    println!("->> {:<12} - handler_hello_world", "HANDLER");

    return Html("Hello <strong>World!</strong>");
}

#[derive(Debug, Deserialize)]
struct HelloParams{
    name: Option<String>,
}

async fn hello_world_query_handler(
    Query(params): Query<HelloParams>
) -> impl IntoResponse {
    let name = params.name.as_deref().unwrap_or("World");
    
    println!("->> {:<12} - handler_hello_world_query - {name:?}", "HANDLER");
    Html(format!("Hello {name}!"))
}

async fn hello_world_path_handler(
    Path(name): Path<String>
) -> impl IntoResponse{
    println!("->> {:<12} - handler_hello_world_path - {name:?}", "HANDLER");

    Html(format!("Hello {name}!"))
}
