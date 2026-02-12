use actix_web::{HttpResponse, Responder, get, web::{self, Data}};
use sqlx::{Database, Encode, Type, query::{QueryAs, QueryScalar}};

use crate::{AppError, AppState};

#[derive(serde::Deserialize)]
struct TimeRange {
    start: Option<i64>,
    end: Option<i64>,
}

impl TimeRange {
    fn as_predicate(&self, start_with_if_present: &str) -> String {
        match (self.start, self.end) {
            (Some(_), Some(_)) => format!("{}st.datetime BETWEEN ? AND ?", start_with_if_present),
            (None, Some(_)) => format!("{}st.datetime < ?", start_with_if_present),
            (Some(_), None) => format!("{}st.datetime > ?", start_with_if_present),
            (None, None) => String::new()
        }
    }
}

trait TimeRangeBindable {
    fn bind_time_range(self, time_range: TimeRange) -> Self;
}

type QAs<'q, DB, O> = QueryAs<'q, DB, O, <DB as Database>::Arguments<'q>>;
type QScalar<'q, DB, O> = QueryScalar<'q, DB, O, <DB as Database>::Arguments<'q>>;

impl <'q, DB: Database, O>TimeRangeBindable for QAs<'q, DB, O> 
    where i64: Encode<'q, DB> + Type<DB>,
{
    fn bind_time_range(self, time_range: TimeRange) -> Self {
        match (time_range.start, time_range.end) {
            (Some(start), Some(end)) => self.bind(start).bind(end),
            (None, Some(end)) => self.bind(end),
            (Some(start), None) => self.bind(start),
            (None, None) => self,
        }
    }
}

impl <'q, DB: Database, O>TimeRangeBindable for QScalar<'q, DB, O> 
    where i64: Encode<'q, DB> + Type<DB>,
{
    fn bind_time_range(self, time_range: TimeRange) -> Self {
        match (time_range.start, time_range.end) {
            (Some(start), Some(end)) => self.bind(start).bind(end),
            (None, Some(end)) => self.bind(end),
            (Some(start), None) => self.bind(start),
            (None, None) => self,
        }
    }
}

#[derive(sqlx::FromRow, serde::Serialize, Debug)]
struct BestSellingProduct {
    id: u32,
    name: String,
    total_sold: u32,
}

#[get("/api/stats/best_selling_product")]
pub async fn best_selling_product(state: Data<AppState>, time_range: web::Query<TimeRange>) -> Result<impl Responder, AppError> {
    let sql = format!(r#"
        SELECT
            p.id,
            p.name,
            SUM(ti.quantity) AS total_sold
        FROM TransactionItem ti
        JOIN StoreTransaction st ON st.id = ti.transaction_id
        JOIN Product p ON p.id = ti.product
        {}
        GROUP BY p.id, p.name
        ORDER BY total_sold DESC
        LIMIT 1
        "#, time_range.as_predicate("WHERE "));
    
    let product: Option<BestSellingProduct> = sqlx::query_as(&sql)
        .bind_time_range(time_range.0).fetch_optional(&state.db).await?;

    Ok(HttpResponse::Ok().json(product))
}

#[derive(sqlx::FromRow, serde::Serialize, Debug)]
struct ProductTransactionInfo {
    count: u32,
    total: f32,
}

#[get("/api/stats/product_transactions")]
pub async fn product_transactions(state: Data<AppState>, time_range: web::Query<TimeRange>) -> Result<impl Responder, AppError> {
    let sql = format!(r#"
        SELECT
            COUNT(*) AS count,
            -COALESCE(SUM(amount), 0.0) AS total
        FROM StoreTransaction
        WHERE amount <= 0 {}
        "#, time_range.as_predicate("AND "));
    let transactions: ProductTransactionInfo = sqlx::query_as(&sql).bind_time_range(time_range.0).fetch_one(&state.db).await?;
    
    Ok(HttpResponse::Ok().json(transactions))
}

#[derive(sqlx::FromRow, serde::Serialize, Debug)]
struct DepositsInfo {
    total: f32,
    average: f32,
}

#[get("/api/stats/deposits")]
pub async fn deposits(state: Data<AppState>, time_range: web::Query<TimeRange>) -> Result<impl Responder, AppError> {
    let sql = format!(r#"
        SELECT
            COALESCE(SUM(st.amount), 0.0) AS total,
            COALESCE(AVG(st.amount), 0.0) AS average
        FROM StoreTransaction st
        WHERE st.amount > 0 {}
        "#, time_range.as_predicate("AND "));
    let info: DepositsInfo = sqlx::query_as(&sql).bind_time_range(time_range.0).fetch_one(&state.db).await?;

    Ok(HttpResponse::Ok().json(info))
}

#[derive(sqlx::FromRow, serde::Serialize, Debug)]
struct CustomerInfo {
    count: u32,
    on_leaderboard: u32,
    private_transactions: u32
}

#[get("/api/stats/customers")]
pub async fn customers(state: Data<AppState>) -> Result<impl Responder, AppError> {
    let sql = r#"
        SELECT
            COUNT(*) AS count,
            SUM(on_leaderboard) AS on_leaderboard,
            SUM(private_transactions) AS private_transactions
        FROM User
        "#;
    let info: CustomerInfo = sqlx::query_as(sql).fetch_one(&state.db).await?;
    
    Ok(HttpResponse::Ok().json(info))
}
