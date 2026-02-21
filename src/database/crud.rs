use sqlx::{QueryBuilder, Result, SqlitePool};
use time::{OffsetDateTime, UtcDateTime};

use crate::database::model::{SwishPaymentRow, TransactionItemRow, TransactionRow};
use crate::model::{PendingTransaction, TransactionDetail, TransactionQuery, TransactionSummary};
use crate::{AppError, Role};

use super::model::UserRow;
use super::model::ProductRow;

//
//          User
//

pub async fn create_user(pool: &SqlitePool, name: Option<&str>, email: &str, google_id: &str) -> Result<UserRow, AppError> {
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

    Ok(UserRow { 
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

pub async fn get_user(pool: &SqlitePool, user_id: Option<u32>, google_id: Option<&str>) -> Result<UserRow, AppError> {
    let user: UserRow = sqlx::query_as(
        r#"
        SELECT id, name, email, google_id, role, balance, on_leaderboard, private_transactions
        FROM User 
        WHERE id = ? OR google_id = ?
        "#).bind(user_id).bind(google_id).fetch_one(pool).await?;
    Ok(user)
}

pub async fn update_user(pool: &SqlitePool, user: UserRow) -> Result<(), AppError> {
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

pub async fn get_users_from_role(pool: &SqlitePool, role: Role) -> Result<Vec<UserRow>, AppError> {
    let users: Vec<UserRow> = sqlx::query_as(
        r#"
        SELECT id, name, email, google_id, role, balance, on_leaderboard, private_transactions
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

/// Returns the created transaction's id 
pub async fn create_transaction(pool: &SqlitePool, transaction: PendingTransaction) -> Result<u32, AppError> {
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
    Ok(id)
}

pub async fn delete_transaction(pool: &SqlitePool, transaction_id: u32) -> Result<(), AppError> {
    sqlx::query(
        r#"
        DELETE FROM StoreTransaction 
        WHERE id = ?
        "#).bind(transaction_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_transaction(pool: &SqlitePool, transaction_id: u32) -> Result<TransactionRow, AppError> {
    let transaction: TransactionRow = sqlx::query_as(r#"
        SELECT id, user, amount, datetime
        FROM StoreTransaction
        WHERE id = ?
        "#).bind(transaction_id).fetch_one(pool).await?;
    
    Ok(transaction)
}

pub async fn get_detailed_transaction(pool: &SqlitePool, transaction_id: u32, user: UserRow) -> Result<TransactionDetail, AppError> {
    let transaction = get_transaction(pool, transaction_id).await?;

    let mut detailed_transaction = TransactionDetail::create(transaction, user);

    let items: Vec<TransactionItemRow> = sqlx::query_as(r#"
        SELECT id, transaction_id, product, quantity, name, price
        FROM TransactionItem
        WHERE transaction_id = ?
        "#).bind(transaction_id).fetch_all(pool).await?;

    detailed_transaction.add_items(items);

    Ok(detailed_transaction)
}

const PAYMENT_KEYWORDS: [&str; 4] = [
    "swish", "insättning", "deposit", "payment"
];

pub async fn query_transactions(pool: &SqlitePool, query: TransactionQuery) -> Result<Vec<TransactionSummary>, AppError> {
    let mut builder = QueryBuilder::new(r#"
        SELECT st.id, u.email AS user_email, st.amount, st.datetime
        FROM StoreTransaction st
        JOIN User u ON u.id = st.user
        WHERE 1=1
        "#);
    
    // OR users
    if !query.user_ids.is_empty() {
        builder.push(" AND st.user IN (");

        let mut sep = builder.separated(", ");
        for id in query.user_ids {
            sep.push_bind(id);
        }
        drop(sep);

        builder.push(")");
    }

    // OR products
    if !query.product_ids.is_empty() {
        builder.push(" AND EXISTS (SELECT 1 FROM TransactionItem item WHERE item.product IN (");

        let mut sep = builder.separated(", ");
        for id in query.product_ids {
            sep.push_bind(id);
        }
        drop(sep);

        builder.push(") AND item.transaction_id = st.id)");
    }

    // BETWEEN time range
    if let Some(time_range) = query.time_range {
        time_range.push_onto_builder(&mut builder, " AND ");
    }
    
    // CURSOR
    if let Some(cursor) = query.cursor {
        let cmp = if query.descending { '<' } else { '>' };
        builder.push(format!(" AND (st.datetime {cmp} ")).push_bind(cursor.datetime);
        builder.push(" OR (st.datetime = ").push_bind(cursor.datetime);
        builder.push(format!(" AND st.id {cmp} ")).push_bind(cursor.id);
        builder.push("))");
    }

    // SEARCH by fts table + keywords
    if let Some(search_term) = query.search_term {

        let (terms, found_payment_keyword) = search_term.split(|c: char| !c.is_alphanumeric())
            .filter(|w| !w.is_empty())
            .map(|w| w.to_lowercase())
            .fold((Vec::new(), false), |(mut terms, mut found_kw), w| {
                if PAYMENT_KEYWORDS.contains(&w.as_str()) {
                    found_kw = true;
                } else {
                    terms.push(format!("\"{w}\"*"));
                }
                (terms, found_kw)
            });
        
        let normalized_search_term = terms.join(" ");

        if !normalized_search_term.trim().is_empty() {
            builder.push(" AND EXISTS (SELECT 1 FROM TransactionFts WHERE transaction_id = st.id");
            builder.push(" AND TransactionFts MATCH ").push_bind(normalized_search_term);
            builder.push(")");
        }

        if found_payment_keyword {
            builder.push(" AND st.amount > 0");
        }
    }

    // ORDER
    if query.descending {
        builder.push(" ORDER BY st.datetime DESC, st.id DESC");
    } else {
        builder.push(" ORDER BY st.datetime ASC, st.id ASC");
    }

    // LIMIT 
    builder.push(" LIMIT ").push_bind(query.limit);
    
    let transactions: Vec<TransactionSummary> = builder.build_query_as().fetch_all(pool).await?;
    
    Ok(transactions)
}

//
//          Payment
//

pub async fn create_payment_request(pool: &SqlitePool, row: SwishPaymentRow) -> Result<(), AppError> {

    sqlx::query(
        r#"
        INSERT INTO SwishPayment (id, user, status, token, location)
        VALUES (?, ?, ?, ?, ?, ?)
        "#
    ).bind(row.id)
    .bind(row.user)
    .bind(row.status)
    .bind(row.token)
    .bind(row.location)
    .execute(pool).await?;

    Ok(())
}
