use crate::models::{NewProduct, Product};
use crate::errors::AppError;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn add_product(
    db: &PgPool,
    category: &str,
    product: NewProduct,
) -> Result<(), AppError> {
    sqlx::query!(
        r#"
        INSERT INTO products (id, name, description, price, image_url, category)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        Uuid::new_v4(),
        product.name,
        product.description,
        product.price,
        product.image_url,
        category
    )
    .execute(db)
    .await?;

    Ok(())
}
