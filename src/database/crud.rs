use sqlx::{Result, SqlitePool};

use crate::types::User;

pub async fn create_user(
    pool: &SqlitePool,
    name: &str,
    personal_number: &str,
) -> Result<u32> {
    // TODO hash personal number
    let id: u32 = sqlx::query_scalar(
        r#"
        INSERT INTO User (name, personal_number, balance)
        VALUES (?, ?, 0)
        RETURNING id
        "#).bind(name).bind(personal_number).fetch_one(pool)
    .await?;
    Ok(id)
}

pub async fn get_user(pool: &SqlitePool, user_id: u32) -> Result<User> {
    sqlx::query_as(
        r#"
        SELECT id, name, balance 
        FROM User 
        WHERE id = ?
        "#).bind(user_id).fetch_one(pool).await
}

pub async fn delete_user(pool: &SqlitePool, user_id: u32) -> Result<()> {
    sqlx::query(
        r#"
        DELETE FROM User WHERE id = ?
        "#).bind(user_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn update_balance(pool: &SqlitePool, user_id: u32, new_balance: f32) -> Result<()> {
    sqlx::query(
        r#"
        UPDATE User SET balance = ? WHERE id = ?
        "#).bind(new_balance).bind(user_id)
    .execute(pool)
    .await?;
    Ok(())
}
