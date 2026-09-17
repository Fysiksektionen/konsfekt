//! This module contains the "models" (structs) we use throughout the codebase.
//! If a struct is used in many files, a good place to put it would be here.

use serde::{Deserialize, Serialize};

use crate::{Role, database::{model::{ProductRow, TransactionItemRow, TransactionRow, UserRow}}, routes::stats};

/// Request body for creating/updating a [`Product`]. All fields are optional so the
/// same struct can be used for partial updates via [`Product::update`]; `name` and
/// `price` are required when creating a new product via [`Product::from_request`].
#[derive(serde::Deserialize)]
pub struct ProductParams {
    pub id: Option<u32>,
    pub name: Option<String>,
    pub price: Option<f32>,
    pub description: Option<String>,
    pub stock: Option<i32>,
    pub flags: Option<ProductFlags>
}

/// A product for sale, as used in memory/route handling. Persisted as [`ProductRow`]
/// via [`Product::into_row`]/[`Product::from_row`].
#[derive(Clone)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub price: f32,
    pub description: String,
    /// `None` means unlimited stock; `Some(n)` is the remaining quantity.
    pub stock: Option<i32>,
    pub flags: ProductFlags,
}

impl Product {
    /// Builds a new [`Product`] (with `id: 0`, to be assigned on insert, and no stock
    /// limit) from a create request.
    ///
    /// # Errors
    /// Returns `Err(())` if `name` or `price` is missing.
    pub fn from_request(params: ProductParams) -> Result<Product, ()> {
        Ok(Product { 
            id: 0,
            name: params.name.ok_or(())?,
            price: params.price.ok_or(())?, 
            description: params.description.unwrap_or("".to_string()),
            stock: None,
            flags: match params.flags {
                Some(flags) => flags,
                None => ProductFlags::default(),
            },
        })
    }

    /// Converts a database row into a [`Product`]. Currently infallible but returns
    /// a `Result` for consistency with [`Product::from_request`].
    pub fn from_row(row: ProductRow) -> Result<Product, ()> {
        Ok(Product { 
            id: row.id,
            name: row.name,
            price: row.price,
            description: row.description,
            stock: row.stock,
            flags: row.flags.0
        })
    }

    /// Applies a partial update in place: any field present in `params` overwrites
    /// the current value. Note `stock` is always overwritten (with `None` clearing it),
    /// unlike the other fields which are left untouched when absent.
    pub fn update(&mut self, params: ProductParams) {
        if let Some(name) = params.name { self.name = name };
        if let Some(price) = params.price { self.price = price };
        if let Some(description) = params.description { self.description = description };
        self.stock = params.stock;

        if let Some(flags) = params.flags {
            self.flags = flags;
        };
    }

    /// Converts this [`Product`] into its database row representation for persistence.
    pub fn into_row(self) -> ProductRow {
        ProductRow {
            id: self.id,
            name: self.name,
            price: self.price,
            description: self.description,
            stock: self.stock,
            flags: sqlx::types::Json(self.flags)
        }
    }

}

/// Display/behavior flags for a [`Product`], stored as JSON in the database.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(default = "ProductFlags::default")]
pub struct ProductFlags {
    pub modifiable: bool, // is only modifiable by admin
    pub new_product: bool,
    pub popular: bool,
    pub marked_sold_out: bool,
}

impl ProductFlags {
    /// Default flags for a freshly created product: modifiable, and not flagged
    /// as new/popular/sold out.
    pub fn default() -> ProductFlags {
        ProductFlags {
            modifiable: true,
            new_product: false,
            popular: false,
            marked_sold_out: false,
        }
    }

    /// Parses [`ProductFlags`] from its JSON string representation.
    pub fn from_str(string: &str) -> Result<ProductFlags, ()> {
        serde_json::from_str::<ProductFlags>(string).map_err(|_| ())
    }

    /// Serializes these flags to a JSON string.
    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

/// A transaction that has been validated (stock checked, total computed) but not
/// yet committed to the database.
pub struct PendingTransaction {
    /// `None` if the purchasing user has `private_transactions` enabled, hiding
    /// their identity from other users' transaction views.
    pub user: Option<u32>, // None if user has private_transactions
    pub amount: f32,
    /// Each purchased product row together with the quantity bought.
    pub products: Vec<(ProductRow, u32)>,
    pub admin_issued: bool
}

/// A single line item within a [`TransactionDetail`] response.
#[derive(serde::Serialize)]
pub struct TransactionItem {
    pub product_id: u32,
    pub name: String,
    pub price: f32,
    pub quantity: u32
}

impl From<TransactionItemRow> for TransactionItem {
    fn from(row: TransactionItemRow) -> Self {
        TransactionItem {
            product_id: row.product,
            name: row.name,
            price: row.price,
            quantity: row.quantity
        }
    }
}

/// Full transaction response, including its line items, as returned to clients.
#[derive(serde::Serialize)]
pub struct TransactionDetail {
    pub id: u32,
    pub amount: f32,
    pub user: Option<UserResponse>, // None if user has private_transactions
    pub datetime: i64,
    pub admin_issued: bool,
    pub items: Vec<TransactionItem>
}

/// Lightweight transaction listing row (no line items), used for paginated lists.
#[derive(serde::Serialize, sqlx::FromRow)]
pub struct TransactionSummary {
    pub id: u32,
    pub amount: f32,
    pub user_email: Option<String>,
    pub admin_issued: bool,
    pub datetime: i64,
}

impl TransactionDetail {
    /// Appends database rows as line items, converting each via [`TransactionItem::from`].
    pub fn add_items(&mut self, items: Vec<TransactionItemRow>) {
        for i in items {
            self.items.push(TransactionItem::from(i));
        }
    }

    /// Builds a [`TransactionDetail`] with no line items yet (use [`Self::add_items`]
    /// to fill them in). The buyer is omitted from the response if they have
    /// `private_transactions` enabled.
    pub fn create(transaction: TransactionRow, user: UserRow) -> Self {
        let user_response = match user.private_transactions {
            true => None,
            false => Some(UserResponse::from(user))
        };
        TransactionDetail {
            id: transaction.id,
            amount: transaction.amount,
            user: user_response,
            datetime: transaction.datetime,
            admin_issued: transaction.admin_issued,
            items: Vec::new()
        }
    }
}

/// Filter/pagination parameters for listing transactions. Empty `user_ids`/`product_ids`
/// mean "no filter" on that dimension.
#[derive(Deserialize)]
pub struct TransactionQuery {
    pub user_ids: Vec<u32>,
    pub product_ids: Vec<u32>,
    pub time_range: Option<stats::TimeRange>,
    pub search_term: Option<String>,
    pub admin_issued: Option<bool>,
    /// Keyset pagination cursor: the last item's `(datetime, id)` from the previous page.
    pub cursor: Option<TimeIdCursor>, // pagination
    pub limit: u32,
    pub descending: bool,
}

/// Keyset pagination cursor pointing at a specific transaction by timestamp and id,
/// used to fetch the next page without an `OFFSET`.
#[derive(Deserialize)]
pub struct TimeIdCursor {
    pub datetime: i64, // UNIX timestamp
    pub id: u32 // e.g Transaction id
}

/// Public representation of a user, as returned to clients.
#[derive(Serialize)]
pub struct UserResponse {
    pub id: u32,
    pub name: Option<String>,
    pub email: String,
    pub balance: f32,
    pub role: Role,
    pub on_leaderboard: bool,
    pub private_transactions: bool
}

impl From<UserRow> for UserResponse {
    fn from(row: UserRow) -> Self {
        UserResponse {
            id: row.id,
            name: row.name,
            email: row.email,
            balance: row.balance,
            role: row.role,
            on_leaderboard: row.on_leaderboard,
            private_transactions: row.private_transactions,
        }
    }
}
