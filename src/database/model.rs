
#[derive(Debug, sqlx::FromRow, serde::Serialize)]
pub struct User {
    pub id: u32,
    pub name: Option<String>,
    pub email: String,
    pub google_id: String,
    pub balance: f32,
    pub role: Option<String>,
}

impl User {
    pub fn get_role(&self) -> Option<Role> {
        match self.role.as_deref() {
            Some("admin") => Some(Role::ADMIN),
            Some("maintainer") => Some(Role::MAINTAINER),
            _ => None
        }
    }
}

pub enum Role {
    ADMIN,
    MAINTAINER,
}
