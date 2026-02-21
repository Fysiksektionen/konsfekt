use serde::{Deserialize, Serialize};

use crate::{Role, database::{model::{ProductRow, TransactionItemRow, TransactionRow, UserRow}}, routes::stats};

#[derive(serde::Deserialize)]
pub struct ProductParams {
    pub id: Option<u32>,
    pub name: Option<String>,
    pub price: Option<f32>,
    pub description: Option<String>,
    pub stock: Option<i32>,
    pub flags: Option<ProductFlags>
}

#[derive(Clone)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub price: f32,
    pub description: String,
    pub stock: Option<i32>,
    pub flags: ProductFlags,
}

impl Product {
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

    pub fn update(&mut self, params: ProductParams) {
        if let Some(name) = params.name { self.name = name };
        if let Some(price) = params.price { self.price = price };
        if let Some(description) = params.description { self.description = description };
        self.stock = params.stock;

        if let Some(flags) = params.flags {
            self.flags = flags;
        };
    }

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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ProductFlags {
    pub modifiable: bool, // is only modifiable by admin
    pub new_product: bool, // Example
    pub marked_sold_out: bool,
}

impl ProductFlags {
    pub fn default() -> ProductFlags {
        ProductFlags { 
            modifiable: true,
            new_product: false,
            marked_sold_out: false,
        }
    }

    pub fn from_str(string: &str) -> Result<ProductFlags, ()> {
        serde_json::from_str::<ProductFlags>(string).map_err(|_| ())
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

pub struct PendingTransaction {
    pub user: u32,
    pub amount: f32,
    pub products: Vec<(ProductRow, u32)>
}

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

#[derive(serde::Serialize)]
pub struct TransactionDetail {
    pub id: u32,
    pub amount: f32,
    pub user: UserResponse,
    pub datetime: i64,
    items: Vec<TransactionItem>
}

#[derive(serde::Serialize, sqlx::FromRow)]
pub struct TransactionSummary {
    pub id: u32,
    pub amount: f32,
    pub user_email: String,
    pub datetime: i64,
}

impl TransactionDetail {
    pub fn add_items(&mut self, items: Vec<TransactionItemRow>) {
        for i in items {
            self.items.push(TransactionItem::from(i));
        }
    }

    pub fn create(transaction: TransactionRow, user: UserRow) -> Self {
        TransactionDetail {
            id: transaction.id,
            amount: transaction.amount,
            user: UserResponse::from(user),
            datetime: transaction.datetime,
            items: Vec::new()
        }
    }
}

#[derive(Deserialize)]
pub struct TransactionQuery {
    pub user_ids: Vec<u32>,
    pub product_ids: Vec<u32>,
    pub time_range: Option<stats::TimeRange>,
    pub search_term: Option<String>,
    pub cursor: Option<TimeIdCursor>, // pagination
    pub limit: u32,
}

#[derive(Deserialize)]
pub struct TimeIdCursor {
    pub datetime: i64, // UNIX timestamp
    pub id: u32 // e.g Transaction id
}

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
