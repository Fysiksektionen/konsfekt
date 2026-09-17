//! Transaction listing/detail routes.

use actix_web::{get, post, web::{self, Data, Json}};

use crate::{AppState, Role, database::crud, error::ApiResult, model::{TransactionDetail, TransactionQuery, TransactionSummary}, return_err, routes::CurrentUser};

/// `GET /api/get_detailed_transaction/{transaction_id}` — fetches a transaction
/// with its line items. Requires an authenticated user ([`Role::User`] or above).
///
/// Note this passes the *requesting* user (not necessarily the transaction's buyer)
/// into [`crud::get_detailed_transaction`], so the response's `user` field and
/// privacy handling are based on the caller, not on who actually made the purchase —
/// this looks unintentional and worth double-checking against how the frontend uses it.
#[get("/api/get_detailed_transaction/{transaction_id}")]
pub async fn get_detailed_transaction(state: Data<AppState>, current_user: CurrentUser, path: web::Path<u32>) -> ApiResult<Json<TransactionDetail>> {
    let user = current_user.require_role(Role::User)?.into_row();
    let transaction = crud::get_detailed_transaction(&state.db, *path, user).await?;
    Ok(Json(transaction))
}

/// Rejects a [`TransactionQuery`] with `403` if a non-admin/maintainer user is
/// requesting transactions that aren't exclusively their own (an empty `user_ids`
/// list, meaning "no filter", also counts as requesting others' transactions).
// Use when/if csv exporting should be implemented
fn check_transaction_query_permission(current_user: CurrentUser, query: &TransactionQuery) -> ApiResult<()> {
    let user = current_user.into_row();

    let other_users_requested = query.user_ids.iter().any(|id| *id != user.id) || query.user_ids.is_empty();
    if user.role == Role::User && other_users_requested {
        return_err!(actix_web::error::ErrorForbidden("Cannot get other user's transactions"));
    }

    Ok(())
}

/// `POST /api/get_transactions` — lists transactions matching a [`TransactionQuery`]
/// filter/pagination body. Regular users may only query their own transactions
/// (see [`check_transaction_query_permission`]); `limit` is clamped to `1..=50`
/// regardless of what the client requests.
#[post("/api/get_transactions")]
pub async fn get_transactions(state: Data<AppState>, current_user: CurrentUser, query: web::Json<TransactionQuery>) -> ApiResult<Json<Vec<TransactionSummary>>> {
    check_transaction_query_permission(current_user, &query.0)?;

    let mut query = query.0;
    query.limit = query.limit.clamp(1, 50);

    let transactions = crud::query_transactions(&state.db, query).await?;

    Ok(Json(transactions))
}
