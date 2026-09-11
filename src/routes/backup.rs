use actix_web::{post, get, web::Data};

use crate::{AppState, Role, database, error::ApiResult, routes::CurrentUser};



#[get("/api/backup/create")]
pub async fn create_backup(state: Data<AppState>, current_user: CurrentUser) -> ApiResult<()> {
    current_user.require_role(Role::Admin)?;

    database::backup::create_backup(&state.db).await?;

    Ok(())
}