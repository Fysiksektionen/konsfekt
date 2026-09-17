//! Row types mirroring the database schema, as returned by [`crate::database::crud`]
//! queries. These are internal/persistence-shaped; client-facing responses are
//! built from them in [`crate::model`] (e.g. [`UserRow`] -> `UserResponse`).

use crate::{Role, model::ProductFlags, routes::payment::swish};

/// A user account row. Includes sensitive fields (`google_id`, `balance`) so it
/// must never be serialized directly to the frontend — see [`crate::model::UserResponse`].
///
/// DO NOT SEND TO FRONTEND
#[derive(Debug, sqlx::FromRow, serde::Serialize)]
pub struct UserRow {
    pub id: u32,
    pub name: Option<String>,
    pub email: String,
    pub google_id: String,
    pub balance: f32,
    pub role: Role,
    pub on_leaderboard: bool,
    pub private_transactions: bool
}

/// A product row as stored in the database.
#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize)]
pub struct ProductRow {
    pub id: u32,
    pub name: String,
    pub price: f32,
    pub description: String,
    /// `None` means unlimited stock.
    pub stock: Option<i32>,
    pub flags: sqlx::types::Json<ProductFlags>,
}


/// A completed transaction row (header only; line items are in [`TransactionItemRow`]).
#[derive(Debug, sqlx::FromRow, serde::Serialize)]
pub struct TransactionRow {
    pub id: u32,
    /// `None` if the buyer had `private_transactions` enabled, or the purchase was anonymous.
    pub user: Option<u32>,
    pub amount: f32,
    pub admin_issued: bool,
    /// UNIX timestamp (seconds).
    pub datetime: i64
}

/// A single purchased product within a [`TransactionRow`]. `name` and `price`
/// are snapshotted at purchase time so historical transactions stay accurate
/// even if the product is later renamed, repriced, or deleted.
#[derive(sqlx::FromRow)]
pub struct TransactionItemRow {
    pub id: u32,
    pub transaction_id: u32,
    pub product: u32,
    pub quantity: u32,
    pub name: String,
    pub price: f32,
}

/// A Swish payment request row, tracking an in-flight or completed Swish payment.
#[derive(sqlx::FromRow)]
pub struct SwishPaymentRequestRow {
    pub id: String, // UUID as readable string
    pub user: u32,
    pub amount: f32,
    pub status: swish::Status,
    /// Bearer token Swish gave us for polling/managing this payment request.
    pub token: String,
    /// Random value embedded in the callback URL, used to reject spoofed callback POSTs.
    pub callback_identifier: String, // Ensure Swish's POST callback is legit
    /// URL Swish returned to poll for status updates on this payment request.
    pub location: String, // Where to poll Swish for new status
}
