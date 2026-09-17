//! Debug-only routes, only registered when `env.is_debug` is true (see `main.rs`'s `create_http`).

use actix_web::{post, web::{self, Data}};

use crate::{AppState, database::crud, error::ApiResult, routes::CurrentUser};

/// Request body for [`add_money`].
#[derive(serde::Deserialize)]
struct MoneyParams { amount: f32 }

/// `POST /api/debug/add_money` — adds `amount` to the current user's balance
/// (debug builds only). Body: `{ "amount": <f32> }`. Requires an authenticated user.
#[post("/api/debug/add_money")]
pub async fn add_money(state: Data<AppState>, current_user: CurrentUser, params: web::Json<MoneyParams>) -> ApiResult<()> {
    let user = current_user.into_row();

    let new_balance = user.balance + params.amount;
    crud::update_user_balance(&state.db, user.id, new_balance).await?;

    Ok(())
}
