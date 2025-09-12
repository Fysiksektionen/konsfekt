use sqlx::{Result, SqlitePool};

#[derive(Debug)]
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
    let rec = sqlx::query(
        r#"
        INSERT INTO User (name, phone_number, balance)
        VALUES (?, ?, 0)
        "#).bind(name).bind(phone_number)
    .execute(pool)
    .await?;

    Ok(rec.last_insert_rowid())
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

/// Read a user’s balance by id
pub async fn get_balance(pool: &SqlitePool, user_id: i64) -> Result<Option<f32>> {
    let balance: Option<f32> = sqlx::query_scalar(
        r#"
        SELECT balance FROM User WHERE id = ?
        "#).bind(user_id).fetch_optional(pool).await?;

    Ok(balance)
}

