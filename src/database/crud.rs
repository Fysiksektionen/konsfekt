use sqlx::{Result, SqlitePool};
use time::{OffsetDateTime, UtcDateTime};

use crate::database::model::{TransactionItemRow, TransactionRow};
use crate::model::{PendingTransaction, Transaction};
use crate::{AppError, Role};

use super::model::User;
use super::model::ProductRow;

//
//          User
//

pub async fn create_user(pool: &SqlitePool, name: Option<&str>, email: &str, google_id: &str) -> Result<User, AppError> {
    let user_table_has_rows: bool = sqlx::query_scalar(r#"
        SELECT EXISTS(SELECT 1 FROM User)"#).fetch_one(pool).await?;
    
    let role = if user_table_has_rows { Role::User } else { Role::Admin };
    let id: u32 = sqlx::query_scalar(
        r#"
        INSERT INTO User (name, email, google_id, role, balance, on_leaderboard, private_transactions)
        VALUES (?, ?, ?, ?, 0, 0, 0)
        RETURNING id
        "#).bind(name).bind(email).bind(google_id).bind(&role).fetch_one(pool)
    .await?;

    Ok(User { 
        id, 
        name: name.map(str::to_owned), 
        email: email.to_string(), 
        google_id: google_id.to_string(),
        role,
        balance: 0.0,
        on_leaderboard: true,
        private_transactions: false
    })
}

pub async fn get_user(pool: &SqlitePool, user_id: Option<u32>, google_id: Option<&str>) -> Result<User, AppError> {
    let user: User = sqlx::query_as(
        r#"
        SELECT id, name, email, google_id, role, balance, on_leaderboard, private_transactions
        FROM User 
        WHERE id = ? OR google_id = ?
        "#).bind(user_id).bind(google_id).fetch_one(pool).await?;
    Ok(user)
}

pub async fn update_user(pool: &SqlitePool, user: User) -> Result<(), AppError> {
    sqlx::query(
        r#"
        UPDATE User SET name = ?, role = ?, balance = ?  
        WHERE id = ?
        "#)
        .bind(user.name)
        .bind(user.role)
        .bind(user.balance)
        .bind(user.id).execute(pool).await?;
    Ok(())
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
    // TODO throw username already exists error if unique conflict
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

pub async fn initiate_email_switch(pool: &SqlitePool, user_id: u32) -> Result<(), AppError> {
    let now = OffsetDateTime::now_utc().unix_timestamp();
    sqlx::query(
        r#"
        INSERT INTO EmailSwitch (user, created_at)
        VALUES (?, ?)
        "#
    ).bind(user_id).bind(now).execute(pool).await?;

    Ok(())
}

pub async fn authorize_email_switch(pool: &SqlitePool, user_id: u32, access_token: &str) -> Result<(), AppError> {
    sqlx::query(
        r#"
        UPDATE EmailSwitch SET access_token = ?
        WHERE user = ?
        "#
    ).bind(access_token).bind(user_id).execute(pool).await?;

    Ok(())
}

pub async fn email_switch_exists(pool: &SqlitePool, user_id: u32) -> Result<bool, AppError> {
    let exists: bool = sqlx::query_scalar(
        r#"
        SELECT EXISTS (
            SELECT 1
            FROM EmailSwitch
            WHERE user = ? AND created_at > strftime('%s', 'now') - 60
        )
        "#
    ).bind(user_id).fetch_one(pool).await?;

    return Ok(exists);
}

pub async fn invalidate_email_switch(pool: &SqlitePool, user_id: u32) -> Result<(), AppError> {
    sqlx::query(
        r#"
        DELETE FROM EmailSwitch
        WHERE user = ?
        "#
    ).bind(user_id).execute(pool).await?;

    Ok(())
}

pub async fn get_users_from_role(pool: &SqlitePool, role: Role) -> Result<Vec<User>, AppError> {
    let users: Vec<User> = sqlx::query_as(
        r#"
        SELECT id, name, email, google_id, role, balance 
        FROM User 
        WHERE role = ?
        "#).bind(role).fetch_all(pool).await?;
    Ok(users)
}

pub async fn finalize_email_switch(pool: &SqlitePool, user_id: u32, new_email: &str, google_id: &str) -> Result<(), AppError> {
    let mut tx = pool.begin().await?;
    sqlx::query(
        r#"
        UPDATE User SET 
            email = ?, 
            google_id = ?
        WHERE id = ?
        "#).bind(new_email).bind(google_id).bind(user_id).execute(&mut *tx).await?;

    sqlx::query(
        r#"
        DELETE FROM EmailSwitch
        WHERE user = ?
        "#
    ).bind(user_id).execute(&mut *tx).await?;

    tx.commit().await?;

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
            stock = ?,
            flags = ?
        WHERE id = ?
        "#)
        .bind(product.name)
        .bind(product.price)
        .bind(product.description)
        .bind(product.stock)
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

pub async fn create_transaction(pool: &SqlitePool, transaction: PendingTransaction) -> Result<(), AppError> {
    let id: u32 = sqlx::query_scalar(
        r#"
        INSERT INTO StoreTransaction (user, amount, datetime)
        VALUES (?, ?, ?)
        RETURNING id
        "#
    ).bind(transaction.user)
    .bind(transaction.amount)
    .bind(UtcDateTime::now().unix_timestamp())
    .fetch_one(pool).await?;
    for (product, quantity) in transaction.products {
        sqlx::query(
            r#"
            INSERT INTO TransactionItem (transaction_id, product, quantity, name, price)
            VALUES (?, ?, ?, ?, ?)
            "#
        ).bind(id)
        .bind(product.id)
        .bind(quantity)
        .bind(product.name)
        .bind(product.price).execute(pool).await?;
    } 
    Ok(())
}

pub async fn get_transactions(pool: &SqlitePool, user_id: Option<u32>) -> Result<Vec<Transaction>, AppError> {
    let sql = format!("
        SELECT id, user, amount, datetime
        FROM StoreTransaction
        {}
        ",
        match user_id {
            Some(_) => "WHERE user = ?",
            None => ""
        });
    let mut query = sqlx::query_as(&sql);

    if let Some(id) = user_id {
        query = query.bind(id);
    }

    let transaction_rows: Vec<TransactionRow> = query.fetch_all(pool).await?;

    let mut transactions = Vec::new();
    for row in transaction_rows {
        let mut transaction = Transaction::from(row);
        let items: Vec<TransactionItemRow> = sqlx::query_as(r#"
            SELECT id, transaction_id, product, quantity, name, price
            FROM TransactionItem
            WHERE transaction_id = ?
            "#).bind(transaction.id).fetch_all(pool).await?;
        transaction.add_items(items);
        transactions.push(transaction);
    }
    Ok(transactions)
}
