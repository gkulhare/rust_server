#![allow(unused)]

use axum::{
    body::Bytes, 
    error_handling::HandleErrorLayer, 
    extract::{DefaultBodyLimit, Path, Query, State}, 
    handler::Handler, http::StatusCode, middleware, 
    response::{Html, IntoResponse, Response}, 
    routing::get, BoxError, Router,
    debug_handler
};
use tower::{
    ServiceBuilder,
    make::Shared
};
use tower_cookies::CookieManagerLayer;
use tower_http::{
    compression::CompressionLayer, limit::RequestBodyLimitLayer, trace::TraceLayer
};
use std::{
    borrow::Cow, collections::HashMap, sync::{Arc,RwLock}, time::Duration
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

    // Add Faux values to initialize the database
    state.write().unwrap().db.insert(String::from("share1"), Bytes::from("Today is gonna be the day"));
    state.write().unwrap().db.insert(String::from("share2"), Bytes::from("That they're gonna throw back to you"));
    state.write().unwrap().db.insert(String::from("share3"), Bytes::from("By now, you should have somehow realized what you've got to do"));

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

fn routes_root(shared_state: SharedState) -> Router{
    Router::new().route(
        "/hello",
        get(|| async { "Hello World"})
    )
    // Route to retrieve a value for a key in
    // the key-store db
    .route(
        "/key/:key",
        get(get_key.layer(CompressionLayer::new()))
        .post(
            set_key
                // .layer((
                //     DefaultBodyLimit::disable(),
                //     RequestBodyLimitLayer::new(1024 * 5_000),
                // ))
            )
    )
    // .layer(
    //     ServiceBuilder::new()
    //         // Handle errors from middleware
    //         .layer(HandleErrorLayer::new(handle_error))
    //         .load_shed()
    //         .concurrency_limit(1024)
    //         .timeout(Duration::from_secs(10))
    //         .layer(TraceLayer::new_for_http()),
    // )
    .with_state(Arc::clone(&shared_state))
}

async fn get_key(
    Path(key): Path<String>,
    State(shared_state): State<SharedState>,
) -> impl IntoResponse {
    
    let db = &shared_state.read().unwrap().db;

    if let Some(value) = db.get(&key){
        Ok(value.clone())
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

#[debug_handler]
async fn set_key(
    Path(key): Path<String>,
    State(shared_state): State<SharedState>,
    bytes: Bytes,
) -> impl IntoResponse{
    shared_state.write().unwrap().db.insert(key, bytes);
}

async fn handle_error(error: BoxError) -> impl IntoResponse {
    if error.is::<tower::timeout::error::Elapsed>() {
        return (StatusCode::REQUEST_TIMEOUT, Cow::from("request timed out"));
    }

    if error.is::<tower::load_shed::error::Overloaded>() {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Cow::from("service is overloaded, try again later"),
        );
    }

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Cow::from(format!("Unhandled internal error: {error}")),
    )
}


