use crate::models::{NewProduct};
use crate::errors::AppError;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn add_product(
    db: &PgPool,
    category: &str,
    product: NewProduct,
) -> Result<(), AppError> {
    sqlx::query(
        "INSERT INTO products (id, name, description, price, image_url, category) VALUES ($1, $2, $3, $4, $5, $6)"
    )
    .bind(Uuid::new_v4())
    .bind(&product.name)
    .bind(&product.description)
    .bind(product.price)
    .bind(&product.image_url)
    .bind(category)
    .execute(db)
    .await?;

    Ok(())
}
