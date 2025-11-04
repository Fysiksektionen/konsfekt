use sqlx::{Result, SqlitePool};

use crate::{AppError, Role};

use super::model::User;

pub async fn create_user(pool: &SqlitePool, name: Option<&str>, email: &str, google_id: &str) -> Result<User, AppError> {
    let user_table_has_rows: bool = sqlx::query_scalar(r#"
        SELECT EXISTS(SELECT 1 FROM User)"#).fetch_one(pool).await?;
   
    let role = if user_table_has_rows { None } else { Some(String::from("admin")) };
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
