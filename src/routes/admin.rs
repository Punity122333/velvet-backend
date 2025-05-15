use axum::{
    extract::{Path, State, Json},
    http::StatusCode,
    response::IntoResponse,
};
use crate::{
    models::{NewProduct},
    state::AppState,
    errors::AppError,
    controllers::product_controller,
};

pub async fn add_product(
    State(state): State<AppState>,
    Path(category): Path<String>,
    Json(product): Json<NewProduct>,
) -> Result<impl IntoResponse, AppError> {
    product_controller::add_product(&state.db, &category, product).await?;
    Ok(StatusCode::CREATED)
}
