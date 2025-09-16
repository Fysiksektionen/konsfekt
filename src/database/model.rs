
#[derive(Debug, sqlx::FromRow, serde::Serialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub balance: f32,
}

#[derive(Debug, sqlx::FromRow)]
pub struct BankidOrder {
    /// orderRef
    pub id: String,
    pub user_id: Option<u32>,
    pub nonce: String,
    pub created_at: i64,
    pub completed_at: Option<i64>,
    pub status: String
}
