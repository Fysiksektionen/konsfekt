//! Aggregate statistics routes (best seller, purchase/deposit/customer totals).
//! All routes here only require a valid session (via [`crate::routes::session_middleware`])
//! and have no additional role check, unlike most other `/api/*` routes.

use actix_web::{get, web::{self, Data}};
use sqlx::{Database, Encode, QueryBuilder, Type, query::{QueryAs, QueryScalar}};

use crate::{AppState, error::{ApiResult, DatabaseError}};

/// An optional `[start, end)`-style UNIX timestamp filter on `StoreTransaction.datetime`,
/// used as a query-string parameter (`?start=<i64>&end=<i64>`) on the stats routes.
/// Either or both bounds may be omitted for an open-ended/unfiltered range.
#[derive(serde::Deserialize)]
pub struct TimeRange {
    start: Option<i64>,
    end: Option<i64>,
}

impl TimeRange {
    /// Builds the SQL predicate fragment for this range (e.g. `"WHERE st.datetime BETWEEN ? AND ?"`),
    /// prefixed with `start_with_if_present` only when the range is non-empty.
    /// Pair with [`TimeRangeBindable::bind_time_range`] to bind the placeholders.
    pub fn as_predicate(&self, start_with_if_present: &str) -> String {
        match (self.start, self.end) {
            (Some(_), Some(_)) => format!("{}st.datetime BETWEEN ? AND ?", start_with_if_present),
            (None, Some(_)) => format!("{}st.datetime < ?", start_with_if_present),
            (Some(_), None) => format!("{}st.datetime > ?", start_with_if_present),
            (None, None) => String::new()
        }
    }

    /// Appends this range's SQL condition (and bound values) directly onto a [`QueryBuilder`],
    /// prefixed with `prefix`. No-op if the range is empty. Used by [`crud::query_transactions`](crate::database::crud::query_transactions).
    pub fn push_onto_builder<'args, DB: Database>(&self, builder: &mut QueryBuilder<'args, DB>, prefix: &str)
    where i64: Encode<'args, DB> + Type<DB>
    {
        match (self.start, self.end) {
            (Some(start), Some(end)) => {
                builder.push(format!("{}st.datetime BETWEEN ", prefix));
                builder.push_bind(start);
                builder.push(" AND ");
                builder.push_bind(end);
            }
            (None, Some(end)) => {
                builder.push(format!("{}st.datetime < ", prefix));
                builder.push_bind(end);
            }
            (Some(start), None) => {
                builder.push(format!("{}st.datetime > ", prefix));
                builder.push_bind(start);
            }
            (None, None) => {}
        }
    }
}

/// Binds whichever of `time_range`'s `start`/`end` are present onto a query type,
/// matching the placeholders produced by [`TimeRange::as_predicate`].
pub trait TimeRangeBindable {
    fn bind_time_range(self, time_range: TimeRange) -> Self;
}

impl <'args, DB: Database>TimeRangeBindable for QueryBuilder<'args, DB>
    where i64: Encode<'args, DB> + Type<DB>,
{
    fn bind_time_range(mut self, time_range: TimeRange) -> Self {
        match (time_range.start, time_range.end) {
            (Some(start), Some(end)) => { self.push_bind(start); self.push_bind(end); }
            (None, Some(end)) => { self.push_bind(end); }
            (Some(start), None) => { self.push_bind(start); }
            (None, None) => {}
        }
        self
    }
}

/// Shorthand for [`sqlx::query::QueryAs`] parameterized over a database's own argument type.
type QAs<'q, DB, O> = QueryAs<'q, DB, O, <DB as Database>::Arguments<'q>>;
/// Shorthand for [`sqlx::query::QueryScalar`] parameterized over a database's own argument type.
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

/// Response body for [`best_selling_product`].
#[derive(sqlx::FromRow, serde::Serialize, Debug)]
struct BestSellingProduct {
    id: u32,
    name: String,
    total_sold: u32,
}

/// `GET /api/stats/best_selling_product?start=<i64>&end=<i64>` — the product with
/// the highest total quantity sold within the (optional) time range, or `null` if
/// there were no sales.
#[get("/api/stats/best_selling_product")]
pub async fn best_selling_product(state: Data<AppState>, time_range: web::Query<TimeRange>) -> ApiResult<web::Json<Option<BestSellingProduct>>> {
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
        .bind_time_range(time_range.0).fetch_optional(&state.db).await
        .map_err(DatabaseError::from)?;

    Ok(web::Json(product))
}

/// Response body for [`purchases`].
#[derive(sqlx::FromRow, serde::Serialize, Debug)]
struct PurchasesInfo {
    count: u32,
    total: f32,
}

/// `GET /api/stats/purchases?start=<i64>&end=<i64>` — count and total value of
/// purchases (transactions with `amount <= 0`) within the (optional) time range.
/// `total` is reported as a positive number (the negated sum of purchase amounts).
#[get("/api/stats/purchases")]
pub async fn purchases(state: Data<AppState>, time_range: web::Query<TimeRange>) -> ApiResult<web::Json<PurchasesInfo>> {
    let sql = format!(r#"
        SELECT
            COUNT(*) AS count,
            -COALESCE(SUM(amount), 0.0) AS total
        FROM StoreTransaction
        WHERE amount <= 0 {}
        "#, time_range.as_predicate("AND "));
    let transactions: PurchasesInfo = sqlx::query_as(&sql)
        .bind_time_range(time_range.0).fetch_one(&state.db).await
        .map_err(DatabaseError::from)?;

    Ok(web::Json(transactions))
}

/// Response body for [`deposits`].
#[derive(sqlx::FromRow, serde::Serialize, Debug)]
struct DepositsInfo {
    total: f32,
    average: f32,
}

/// `GET /api/stats/deposits?start=<i64>&end=<i64>` — total and average value of
/// deposits (transactions with `amount > 0`) within the (optional) time range.
#[get("/api/stats/deposits")]
pub async fn deposits(state: Data<AppState>, time_range: web::Query<TimeRange>) -> ApiResult<web::Json<DepositsInfo>> {
    let sql = format!(r#"
        SELECT
            COALESCE(SUM(st.amount), 0.0) AS total,
            COALESCE(AVG(st.amount), 0.0) AS average
        FROM StoreTransaction st
        WHERE st.amount > 0 {}
        "#, time_range.as_predicate("AND "));
    let info: DepositsInfo = sqlx::query_as(&sql)
        .bind_time_range(time_range.0).fetch_one(&state.db).await
        .map_err(DatabaseError::from)?;

    Ok(web::Json(info))
}

/// Response body for [`customers`].
#[derive(sqlx::FromRow, serde::Serialize, Debug)]
struct CustomerInfo {
    count: u32,
    on_leaderboard: u32,
    private_transactions: u32
}

/// `GET /api/stats/customers` — total user count, and how many have
/// `on_leaderboard`/`private_transactions` enabled.
#[get("/api/stats/customers")]
pub async fn customers(state: Data<AppState>) -> ApiResult<web::Json<CustomerInfo>> {
    let sql = r#"
        SELECT
            COUNT(*) AS count,
            SUM(on_leaderboard) AS on_leaderboard,
            SUM(private_transactions) AS private_transactions
        FROM User
        "#;
    let info: CustomerInfo = sqlx::query_as(sql).fetch_one(&state.db).await
        .map_err(DatabaseError::from)?;

    Ok(web::Json(info))
}
