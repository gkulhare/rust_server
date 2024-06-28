#![allow(unused)]

use axum::{
    body::Bytes, extract::{Path, Query, State}, middleware, response::{Html, IntoResponse, Response}, routing::get, Router, http::StatusCode,
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

    let state = SharedState::default();

    let router = Router::new()
    .merge(routes_root(Arc::clone(&state)))
    .layer(middleware::map_response(main_response_mapper))
    .layer(CookieManagerLayer::new());
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("->> LISTENING ON localhost:8080\n");
    axum::serve(listener, router).await.unwrap();
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");

    println!();
    res
}

fn routes_root(sharedstate: SharedState) -> Router{
    Router::new().route(
        "/hello",
        get(|| async { "Hello World"})
    )
    // Route to retrieve a value for a key in
    // the key-store db
    .route(
        "key/:key",
        get(get_key)
    )
    .with_state(Arc::clone(&sharedstate))
}

async fn get_key(
    Path(key): Path<String>,
    State(sharedstate): State<SharedState>,
) -> impl IntoResponse {
    
    let db = &sharedstate.read().unwrap().db;

    if let Some(value) = db.get(&key){
        Ok(value.clone())
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}



