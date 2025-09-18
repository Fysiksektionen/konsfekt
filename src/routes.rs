
use actix_web::{get, HttpResponse, Responder};

#[get("/api")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello backend!")
}
