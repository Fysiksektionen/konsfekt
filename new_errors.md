### Old errors
```rust 
Err(AppError::BadRequest(String::from("Bad Request")))
Err(AppError::ActixError(actix_web::error::ErrorUnauthorized("Cannot get other user's information")));
Err(AppError::GenericError(String::from("Couldn't parse cookie")));

pub async fn get_user(pool: &SqlitePool, user_id: Option<u32>, google_id: Option<&str>) -> Result<UserRow, AppError> {
    let user: UserRow = sqlx::query_as(
        r#"
        SELECT id, name, email, google_id, role, balance, on_leaderboard, private_transactions
        FROM User 
        WHERE id = ? OR google_id = ?
        "#).bind(user_id).bind(google_id).fetch_one(pool).await?;
    Ok(user)
}
```

### New errors
```rust
use actix_web::error as actix_error;
Err(actix_error::BadRequest("Request is in wrong format"))
Err(actix_error::ErrorUnauthorized("Cannot get other user's information"))
Err(actix_error::BadRequest("Couldn't parse cookie"))

struct DatabaseError(sqlx::Error)

impl From<sqlx::Error> for DatabaseError {
    fn from(err: sqlx::Error) -> Self {
        DatabaseError(err)
    }
}

impl ResponseError for DatabaseError ...

pub async fn get_user(pool: &SqlitePool, user_id: Option<u32>, google_id: Option<&str>) -> Result<UserRow, DatabaseError> {
    let user: UserRow = sqlx::query_as(
        r#"
        SELECT id, name, email, google_id, role, balance, on_leaderboard, private_transactions
        FROM User 
        WHERE id = ? OR google_id = ?
        "#).bind(user_id).bind(google_id).fetch_one(pool).await?;
    Ok(user)
}

Err(SwishError(response.json().await?))

```

