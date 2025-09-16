use actix_web::web::Data;
use sqlx::{Result, SqlitePool};

use crate::{auth, AppState};

use super::model::User;

async fn create_user(
    pool: &SqlitePool,
    name: &str,
    personal_number: &str,
) -> Result<User> {
    // TODO hash personal number
    let id: u32 = sqlx::query_scalar(
        r#"
        INSERT INTO User (name, personal_number, balance)
        VALUES (?, ?, 0)
        RETURNING id
        "#).bind(name).bind(personal_number).fetch_one(pool)
    .await?;

    Ok(User { 
        id, 
        name: name.to_string(), 
        balance: 0.0
    })
}

pub async fn get_or_create_user(state: &Data<AppState>, name: &str, personal_number: &str) -> Result<User> {
    if let Ok(user) = get_user(state.clone(), None, Some(personal_number.to_string())).await {
        return Ok(user);
    }
    create_user(&state.db, name, personal_number).await
}

pub async fn get_user(state: Data<AppState>, user_id: Option<u32>, personal_number: Option<String>) -> Result<User> {
    let hash = match personal_number {
        Some(pn) => auth::hash_personal_number(&state.env_vars.hmac_secret, &pn),
        None => String::new()
    };

    sqlx::query_as(
        r#"
        SELECT id, name, balance 
        FROM User 
        WHERE id = ? OR personal_number = ?
        "#).bind(user_id).bind(hash).fetch_one(&state.db).await
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
