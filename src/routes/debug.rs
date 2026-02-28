use actix_web::{HttpRequest, HttpResponse, Responder, ResponseError, post, web::{self, Data}};

use crate::{AppState, database::crud, routes::user_from_cookie};

#[derive(serde::Deserialize)]
struct MoneyParams { amount: f32 }

#[post("/api/debug/add_money")]
pub async fn add_money(state: Data<AppState>, req: HttpRequest, params: web::Json<MoneyParams>) -> Result<impl Responder, impl ResponseError> {
    let user = user_from_cookie(&state.db, &req).await?;
    let new_balance = user.balance + params.amount;
    crud::update_user_balance(&state.db, user.id, new_balance).await?;

    Ok(HttpResponse::Ok())
}
