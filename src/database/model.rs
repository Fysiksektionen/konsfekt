use crate::{Role, model::ProductFlags};

#[derive(Debug, sqlx::FromRow, serde::Serialize)]
pub struct User {
    pub id: u32,
    pub name: Option<String>,
    pub email: String,
    pub google_id: String,
    pub balance: f32,
    pub role: Role,
}

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize)]
pub struct ProductRow {
    pub id: u32,
    pub name: String,
    pub price: f32,
    pub description: String,
    pub stock: Option<i32>,
    pub flags: sqlx::types::Json<ProductFlags>,
}


#[derive(Debug, sqlx::FromRow, serde::Serialize)]
pub struct TransactionRow {
    pub id: u32,
    pub user: u32,
    pub total_price: f32,
    pub datetime: i64
}
