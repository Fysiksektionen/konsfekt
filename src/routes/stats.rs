use actix_web::{HttpResponse, Responder, get, web::{self, Data}};
use sqlx::{Database, Encode, FromRow, Type, query::QueryAs};

use crate::{AppError, AppState};

#[derive(serde::Deserialize)]
struct TimeRange {
    start: Option<i64>,
    end: Option<i64>,
}

impl TimeRange {
    fn get_where_query(&self) -> String {
        match (self.start, self.end) {
            (Some(_), Some(_)) => format!("WHERE st.datetime BETWEEN ? AND ?"),
            (None, Some(_)) => format!("WHERE st.datetime < ?"),
            (Some(_), None) => format!("WHERE st.datetime > ?"),
            (None, None) => String::new()
        }
    }

    fn bind_to<'q, DB, O>(
        &self,
        query: QueryAs<'q, DB, O, <DB as Database>::Arguments<'q>>,
    ) -> QueryAs<'q, DB, O, <DB as Database>::Arguments<'q>>
    where
        DB: Database,
        O: for<'r> FromRow<'r, DB::Row>,
        i64: Encode<'q, DB> + Type<DB>,

    {        match (self.start, self.end) {
            (Some(start), Some(end)) => query.bind(start).bind(end),
            (None, Some(end)) => query.bind(end),
            (Some(start), None) => query.bind(start),
            (None, None) => query,
        }
    }
}

#[derive(sqlx::FromRow, serde::Serialize)]
struct BestSellingProduct {
    id: u32,
    name: String,
    total_sold: u32,
}

#[get("/api/stats/get_best_selling_product")]
pub async fn get_best_selling_product(state: Data<AppState>, query: web::Query<TimeRange>) -> Result<impl Responder, AppError> {
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
        "#, query.get_where_query());
    let query = query.bind_to(sqlx::query_as(&sql));
    let product: Option<BestSellingProduct> = query.fetch_optional(&state.db).await?;

    Ok(HttpResponse::Ok().json(product))
}
