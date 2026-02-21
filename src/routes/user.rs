use actix_web::{HttpRequest, HttpResponse, Responder, get, post, web::{self, Data}};
use serde::{Deserialize, Serialize};

use crate::{AppError, AppState, Role, database::{crud, model::UserRow}, model::{TransactionQuery, UserResponse}, routes::user_from_cookie};

#[get("/api/get_user")]
pub async fn get_user(state: Data<AppState>, req: HttpRequest) -> Result<web::Json<UserResponse>, AppError> {
    let user = user_from_cookie(&state.db, &req).await?;
    
    let user_response =  UserResponse {
        id: user.id,
        name: user.name,
        email: user.email,
        balance: user.balance,
        role: user.role,
        on_leaderboard: user.on_leaderboard,
        private_transactions: user.private_transactions
    };
    Ok(web::Json(user_response))
}

#[derive(Deserialize)]
struct GetUsersQuery {
    role: Option<String>
}

#[derive(Serialize)]
struct GetUsersResponse {
    users: Vec<UserRow>
}

#[get("/api/get_users")]
pub async fn get_users(state: Data<AppState>, req: HttpRequest, query: web::Query<GetUsersQuery>) -> Result<web::Json<GetUsersResponse>, AppError> {
    // Possible to expand in futre for diffrent queries

    let user = user_from_cookie(&state.db, &req).await?;
    if user.role <= Role::Maintainer {
        return Err(AppError::ActixError(actix_web::error::ErrorUnauthorized("Cannot get other user's information")));
    }

    match &query.role {
        Some(role) => {
            let users = crud::get_users_from_role(&state.db, Role::from_str(role.as_str())).await?;
            Ok(web::Json(GetUsersResponse { users: users }))
        },
        None => Err(AppError::BadRequest(String::from("Bad Request")))
    }
}

#[derive(Deserialize)]
struct DeleteUserQuery {
    id: u32,  
}

#[post("/api/delete_user")]
pub async fn delete_user(state: Data<AppState>, req: HttpRequest, query: web::Query<DeleteUserQuery>) -> Result<(), AppError> {
    
    let user = user_from_cookie(&state.db, &req).await?;
    if user.role <= Role::Maintainer {
        return Err(AppError::ActixError(actix_web::error::ErrorUnauthorized("Cannot delete other users")));
    }
    
    crud::delete_user(&state.db, query.id).await?;

    Ok(())
}

#[derive(Deserialize)]
struct UpdateUserParams {
    id: u32,
    name: Option<String>,
    balance: Option<f32>,
    role: Option<Role>
}

#[derive(Deserialize)]
struct ChangeUsernameParam {
    name: String,
}

#[post("/api/set_username")]
pub async fn set_username(state: Data<AppState>, req: HttpRequest, params: web::Json<ChangeUsernameParam>) -> Result<(), AppError> { 
    let user = user_from_cookie(&state.db, &req).await?;
    crud::update_user_name(&state.db, user.id, &params.name).await?;

    Ok(())
}

#[post("/api/update_user")]
pub async fn update_user(state: Data<AppState>, req: HttpRequest, params: web::Json<UpdateUserParams>) -> Result<(), AppError> {
    let user_admin = user_from_cookie(&state.db, &req).await?;
    if user_admin.role <= Role::Maintainer {
        return Err(AppError::ActixError(actix_web::error::ErrorUnauthorized("Cannot change other user's information")));
    }
    
    let mut user = crud::get_user(&state.db, Some(params.id), None).await?;
    if user.role == Role::Admin && user_admin.role != Role::Admin {
        return Err(AppError::ActixError(actix_web::error::ErrorUnauthorized("Cannot change an admins information")));
    }
    if let Some(role) = params.role { user.role = role };
    if let Some(balance) = params.balance { user.balance = balance };
    user.name = params.name.clone();
    
    crud::update_user(&state.db, user).await?;

    Ok(())
}

#[get("/api/get_detailed_transaction/{transaction_id}")]
pub async fn get_detailed_transaction(state: Data<AppState>, req: HttpRequest, path: web::Path<u32>) -> Result<impl Responder, AppError> {
    let user = user_from_cookie(&state.db, &req).await?;

    let transaction = crud::get_detailed_transaction(&state.db, *path, user).await?;
    
    Ok(HttpResponse::Ok().json(transaction))
}

#[post("/api/get_transactions")]
pub async fn get_transactions(state: Data<AppState>, req: HttpRequest, query: web::Json<TransactionQuery>) -> Result<impl Responder, AppError> {
    let user = user_from_cookie(&state.db, &req).await?;

    let other_users_requested = query.user_ids.iter().any(|id| *id != user.id) || query.user_ids.is_empty();
    if user.role == Role::User && other_users_requested {
        return Err(AppError::ActixError(actix_web::error::ErrorUnauthorized("Cannot get other user's transactions")));
    }

    let transactions = crud::query_transactions(&state.db, query.0).await?;

    Ok(HttpResponse::Ok().json(transactions))
}
