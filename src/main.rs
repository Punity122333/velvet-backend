mod auth;
mod db;
mod models;
mod errors;
mod state;
mod routes;
mod controllers;

use axum::{response::IntoResponse, routing::{get, post}, Router};
use dotenv::dotenv;
use state::AppState;
use routes::admin::add_product;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};




async fn root_handler() -> impl IntoResponse {
    "Hello, world!"
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let db = db::connect_db().await;
    let state = AppState { db: db.into() };

    let app = Router::new()
        .route("/admin/products/:category", post(add_product))
        .route("/", get(root_handler))
        .layer(cors)
        .with_state(state);

    let addr = "0.0.0.0:3000";
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}