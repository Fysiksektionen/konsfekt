//! Manual database backup route.

use actix_web::{get, web::Data};

use crate::{AppState, Role, database, error::ApiResult, routes::CurrentUser};



/// `GET /api/backup/create` — triggers an immediate database backup.
///
/// Requires [`Role::Admin`]. Responds `200` with no body on success.
#[get("/api/backup/create")]
pub async fn create_backup(state: Data<AppState>, current_user: CurrentUser) -> ApiResult<()> {
    current_user.require_role(Role::Admin)?;

    database::backup::create_backup(&state.db).await?;

    Ok(())
}
