use sqlx::{prelude::FromRow, Result, SqlitePool};

#[derive(Debug, FromRow)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub phone_number: String,
    pub balance: f32,
}

/// Create a new user
pub async fn create_user(
    pool: &SqlitePool,
    name: &str,
    phone_number: &str,
) -> Result<i64> {
    let res = sqlx::query(
        r#"
        INSERT INTO User (name, phone_number, balance)
        VALUES (?, ?, 0)
        "#).bind(name).bind(phone_number)
    .execute(pool)
    .await?;

    Ok(res.last_insert_rowid())
}

pub async fn get_user(pool: &SqlitePool, user_id: Option<i64>, phone_number: Option<String>) -> Result<User> {
    sqlx::query_as(
        r#"
        SELECT id, name, phone_number, balance 
        FROM User 
        WHERE id = ? OR phone_number = ?
        "#).bind(user_id).bind(phone_number).fetch_one(pool).await
}

/// Delete a user by id
pub async fn delete_user(pool: &SqlitePool, user_id: i64) -> Result<u64> {
    let res = sqlx::query(
        r#"
        DELETE FROM User WHERE id = ?
        "#).bind(user_id)
    .execute(pool)
    .await?;

    Ok(res.rows_affected())
}

/// Update a user’s balance
pub async fn update_balance(pool: &SqlitePool, user_id: i64, new_balance: f32) -> Result<u64> {
    let res = sqlx::query(
        r#"
        UPDATE User SET balance = ? WHERE id = ?
        "#).bind(new_balance).bind(user_id)
    .execute(pool)
    .await?;

    Ok(res.rows_affected())
}
