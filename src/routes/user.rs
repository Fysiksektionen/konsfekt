
use actix_web::{Result, get, post, web::{self, Data}};
use serde::{Deserialize, Serialize};

use crate::{AppState, Role, database::{crud, model::UserRow}, error::ApiResult, model::{PendingTransaction, UserResponse}, return_err, routes::{CurrentUser}};

#[get("/api/get_user")]
pub async fn get_user(current_user: CurrentUser) -> Result<web::Json<UserResponse>, actix_web::Error> {
    let user = current_user.require_role(Role::User)?.into_row();
    
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
pub async fn get_users(state: Data<AppState>, current_user: CurrentUser, query: web::Query<GetUsersQuery>) -> ApiResult<web::Json<GetUsersResponse>> {
    // Possible to expand in future for diffrent queries

    current_user.require_role(Role::Maintainer)?;
    
    match &query.role {
        Some(role) => {
            let users = crud::get_users_from_role(&state.db, Role::from_str(role.as_str())).await?;
            Ok(web::Json(GetUsersResponse { users: users }))
        },
        None => { return_err!(actix_web::error::ErrorBadRequest("")); }
    }
}

#[derive(Deserialize)]
struct DeleteUserQuery {
    id: u32,  
}

#[post("/api/delete_user")]
pub async fn delete_user(state: Data<AppState>, current_user: CurrentUser, query: web::Query<DeleteUserQuery>) -> ApiResult<()> {
    current_user.require_role(Role::Maintainer)?;
    crud::delete_user(&state.db, query.id).await?;

    Ok(())
}

#[derive(Deserialize)]
struct UpdateUserParams {
    id: u32,
    name: Option<String>,
    balance: Option<f32>,
    role: Option<Role>,
}

#[derive(Deserialize)]
struct ChangeUsernameParam {
    name: String,
}

#[post("/api/set_username")]
pub async fn set_username(state: Data<AppState>, current_user: CurrentUser, params: web::Json<ChangeUsernameParam>) -> ApiResult<()> { 
    let user = current_user.require_role(Role::User)?.into_row();
    crud::update_user_name(&state.db, user.id, &params.name).await?;

    Ok(())
}

#[derive(Deserialize)]
struct UserFlagsQueryParams {
    private_transactions: Option<bool>,
    on_leaderboard: Option<bool>
}

#[post("/api/set_user_flags")]
pub async fn set_user_flags(state: Data<AppState>, current_user: CurrentUser, flags: web::Query<UserFlagsQueryParams>) -> ApiResult<()> {
    let user = current_user.require_role(Role::User)?.into_row();

    if let Some(private_transactions) = flags.private_transactions {
        crud::set_private_transactions(&state.db, user.id, private_transactions).await?;
    }
    if let Some(on_leaderboard) = flags.on_leaderboard {
        crud::set_on_leaderboard(&state.db, user.id, on_leaderboard).await?;
    }

    Ok(())
}

#[post("/api/update_user")]
pub async fn update_user(current_user: CurrentUser, state: Data<AppState>, params: web::Json<UpdateUserParams>) -> ApiResult<()> {
    let user_admin = current_user.require_role(Role::Maintainer)?.into_row();

    let mut user = crud::get_user(&state.db, Some(params.id), None).await?;
    if user.role == Role::Admin && user_admin.role != Role::Admin {
        return_err!(actix_web::error::ErrorForbidden("Cannot change an admins information"));
    }

    if let Some(role) = params.role { user.role = role };
    if let Some(balance) = params.balance { 
        if balance != user.balance {
            let transaction = PendingTransaction {
                user: Some(user.id), 
                amount: balance - user.balance, 
                products: Vec::new(),
                admin_issued: true
            };
            crud::create_transaction(&state.db, transaction).await?;
        }
        user.balance = balance 
    };
    user.name = params.name.clone();
    
    crud::update_user(&state.db, user).await?;

    Ok(())
}

#[post("/api/unlink_transactions")]
pub async fn unlink_transactions(state: Data<AppState>, current_user: CurrentUser) -> ApiResult<()> {
    let user = current_user.require_role(Role::User)?.into_row();
    crud::unlink_transactions(&state.db, user.id).await?;

    Ok(())
}
