use actix_web::web::Data;
use serde::Deserialize;

use crate::{database::crud, AppError, AppState};

/// Response when calling /auth on BankID's API
#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderResponse {
    pub order_ref: String,
    pub auto_start_token: String,
    pub qr_start_token: String,
    pub qr_start_secret: String
}

/// Response when calling /collect on BankID's API
/// https://developers.bankid.com/api-references/auth--sign/collect
#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CollectResponse {
    pub order_ref: String,
    pub status: String,
    pub completion_data: Option<CompletionData>,
}

impl CollectResponse {
    pub async fn get_user(&self, state: &Data<AppState>) -> Result<Option<crud::User>, AppError> {
        if self.status == "complete" {
            if let Some(data) = &self.completion_data {
                return Ok(Some(crud::get_or_create_user(state, &data.user.name, &data.user.personal_number).await?));
            }
        }
        Ok(None)
    }
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CompletionData {
    pub user: User
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub name: String,
    pub personal_number: String,
}
