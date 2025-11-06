use crate::Role;

#[derive(Debug, sqlx::FromRow, serde::Serialize)]
pub struct User {
    pub id: u32,
    pub name: Option<String>,
    pub email: String,
    pub google_id: String,
    pub balance: f32,
    pub role: Role,
}

#[derive(Debug, sqlx::FromRow, serde::Serialize)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub price: f32,
    pub description: String,
    pub stock: Option<i32>,
}

#[derive(Debug, sqlx::FromRow, serde::Serialize)]
pub struct Transaction {
    pub id: u32,
    pub product: Product,
    pub user: User,
    pub amount: f32,
}