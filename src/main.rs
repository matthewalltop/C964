extern crate core;

mod algo;
mod api;
mod enums;
mod frames;
mod requests;
mod plots;
mod predict;
mod traits;

pub(crate) type JsonResponse = Result<String, Box<dyn  Error>>;


use std::error::Error;
use axum::{routing::{get}, Router};
use axum::http::{Method};
use axum::routing::post;
use tower::ServiceBuilder;
use tower_http::cors::{AllowOrigin, CorsLayer};
use crate::api::{demographic_handler, predict_handler, mental_health_handler};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let cors_layer = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_origin(AllowOrigin::any());
    
    // TODO: Revisit this & restrict its use via config.
    // https://github.com/tokio-rs/axum/blob/main/examples/cors/src/main.rs
    // .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
    
    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .route("/demographics", get(demographic_handler))
        .route("/mental-health", get(mental_health_handler))
        .route("/predict", post(predict_handler))
        .layer(ServiceBuilder::new()
            .layer(cors_layer));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root () -> &'static str {
    // TODO: Redirect to frontend URI.
    "Hello World!"
}