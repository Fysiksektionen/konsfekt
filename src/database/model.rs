
#[derive(Debug, sqlx::FromRow, serde::Serialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub balance: f32,
}
