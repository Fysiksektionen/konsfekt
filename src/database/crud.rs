use sqlx::{Result, SqlitePool};
use time::UtcDateTime;

use crate::{AppError, Role};

use super::model::User;
use super::model::ProductRow;
use super::model::Transaction;

//
//          User
//

pub async fn create_user(pool: &SqlitePool, name: Option<&str>, email: &str, google_id: &str) -> Result<User, AppError> {
    let user_table_has_rows: bool = sqlx::query_scalar(r#"
        SELECT EXISTS(SELECT 1 FROM User)"#).fetch_one(pool).await?;
    
    let role = if user_table_has_rows { Role::User } else { Role::Admin };
    let id: u32 = sqlx::query_scalar(
        r#"
        INSERT INTO User (name, email, google_id, role, balance)
        VALUES (?, ?, ?, ?, 0)
        RETURNING id
        "#).bind(name).bind(email).bind(google_id).bind(&role).fetch_one(pool)
    .await?;

    Ok(User { 
        id, 
        name: name.map(str::to_owned), 
        email: email.to_string(), 
        google_id: google_id.to_string(),
        role,
        balance: 0.0
    })
}

pub async fn get_user(pool: &SqlitePool, user_id: Option<u32>, google_id: Option<&str>) -> Result<User, AppError> {
    let user: User = sqlx::query_as(
        r#"
        SELECT id, name, email, google_id, role, balance 
        FROM User 
        WHERE id = ? OR google_id = ?
        "#).bind(user_id).bind(google_id).fetch_one(pool).await?;
    Ok(user)
}

pub async fn delete_user(pool: &SqlitePool, user_id: u32) -> Result<(), AppError> {
    sqlx::query(
        r#"
        DELETE FROM User 
        WHERE id = ?
        "#).bind(user_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn update_user_name(pool: &SqlitePool, user_id: u32, new_name: &str) -> Result<(), AppError> {
    sqlx::query(
        r#"
        UPDATE User SET name = ? 
        WHERE id = ?
        "#).bind(new_name).bind(user_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn update_user_balance(pool: &SqlitePool, user_id: u32, new_balance: f32) -> Result<(), AppError> {
    sqlx::query(
        r#"
        UPDATE User SET balance = ? 
        WHERE id = ?
        "#).bind(new_balance).bind(user_id)
    .execute(pool)
    .await?;
    Ok(())
}

//
//          Shop
//

pub async fn create_product(pool: &SqlitePool, mut product: ProductRow) -> Result<ProductRow, AppError> {
    
    let id: u32 = sqlx::query_scalar(
        r#"
        INSERT INTO Product (name, price, description, flags)
        VALUES (?, ?, ?, ?)
        RETURNING id
        "#
    ).bind(product.name.clone())
    .bind(product.price)
    .bind(product.description.clone())
    .bind(product.flags.clone())
    .fetch_one(pool).await?;

    product.id = id;

    Ok(product)
}

pub async fn get_product(pool: &SqlitePool, id: u32) -> Result<ProductRow, AppError> {
    let product: ProductRow = sqlx::query_as(
        r#"
        SELECT id, name, price, description, stock, flags
        FROM Product 
        WHERE id = ?
        "#).bind(id).fetch_one(pool).await?;
    Ok(product)
}

pub async fn get_products(pool: &SqlitePool) -> Result<Vec<ProductRow>, AppError> {
    let products: Vec<ProductRow> = sqlx::query_as(
        r#"
        SELECT id, name, price, description, stock, flags
        FROM Product
        ORDER BY id DESC
        "#).fetch_all(pool).await?;

    Ok(products)
}

pub async fn update_product_data(pool: &SqlitePool, product: ProductRow) -> Result<(), AppError> {
    sqlx::query(
        r#"
        UPDATE Product SET 
            name = ?, 
            price = ?, 
            description = ?,
            flags = ?
        WHERE id = ?
        "#)
        .bind(product.name)
        .bind(product.price)
        .bind(product.description)
        .bind(product.flags)
        .bind(product.id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn update_product_stock(pool: &SqlitePool, id: u32, stock: Option<i32>) -> Result<(), AppError> {
    sqlx::query(
        r#"
        UPDATE Product SET 
            stock = ?
        WHERE id = ?
        "#)
        .bind(stock)
        .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn delete_product(pool: &SqlitePool, id: u32) -> Result<(), AppError> {
    sqlx::query(
        r#"
        DELETE FROM Product 
        WHERE id = ?
        "#
    ).bind(id).execute(pool).await?;

    Ok(())
}

pub async fn create_transaction(pool: &SqlitePool, mut transaction: Transaction) -> Result<Transaction, AppError> {
    let id: u32 = sqlx::query_scalar(
        r#"
        INSERT INTO StoreTransaction (product, user, amount, datetime)
        VALUES (?, ?, ?, ?)
        RETURNING id
        "#
    ).bind(transaction.product)
    .bind(transaction.user)
    .bind(transaction.amount)
    .bind(UtcDateTime::now().unix_timestamp())
    .fetch_one(pool).await?;
    
    transaction.id = id;

    Ok(transaction)
}