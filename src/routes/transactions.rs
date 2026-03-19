use actix_web::{HttpRequest, get, post, web::{self, Data, Json}};

use crate::{AppState, Role, database::crud, error::ApiResult, model::{TransactionDetail, TransactionQuery, TransactionSummary}, return_err, routes::user_from_cookie};

#[get("/api/get_detailed_transaction/{transaction_id}")]
pub async fn get_detailed_transaction(state: Data<AppState>, req: HttpRequest, path: web::Path<u32>) -> ApiResult<Json<TransactionDetail>> {
    let user = user_from_cookie(&state.db, &req).await?;
    let transaction = crud::get_detailed_transaction(&state.db, *path, user).await?;
    Ok(Json(transaction))
}

// Use when/if csv exporting should be implemented
async fn check_transaction_query_permission(state: &Data<AppState>, req: HttpRequest, query: &TransactionQuery) -> ApiResult<()> {
    let user = user_from_cookie(&state.db, &req).await?;
    let other_users_requested = query.user_ids.iter().any(|id| *id != user.id) || query.user_ids.is_empty();
    if user.role == Role::User && other_users_requested {
        return_err!(actix_web::error::ErrorUnauthorized("Cannot get other user's transactions"));
    }
    Ok(())
}

#[post("/api/get_transactions")]
pub async fn get_transactions(state: Data<AppState>, req: HttpRequest, query: web::Json<TransactionQuery>) -> ApiResult<Json<Vec<TransactionSummary>>> {
    check_transaction_query_permission(&state, req, &query.0).await?;

    let mut query = query.0;
    query.limit = query.limit.clamp(1, 50);

    let transactions = crud::query_transactions(&state.db, query).await?;

    Ok(Json(transactions))
}
