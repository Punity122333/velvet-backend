mod auth;
mod db;
mod models;
mod errors;
mod state;
mod routes;
mod controllers;

use axum::{Router, routing::post};
use dotenv::dotenv;
use state::AppState;
use routes::admin::add_product;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db = db::connect_db().await;
    let state = AppState { db: db.into() };

    let app = Router::new()
        .route("/admin/products/:category", post(add_product))
        .with_state(state);

    let addr = "0.0.0.0:3000";
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}