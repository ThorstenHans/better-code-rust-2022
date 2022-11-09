use actix_web::{get, Responder, HttpResponse};
use super::models::HealthEndpointResponseModel;

#[get("/readiness")]
pub async fn readiness() -> impl Responder {
    let h = HealthEndpointResponseModel::new();
    HttpResponse::Ok().json(h)
}

#[get("/liveness")]
pub  async fn liveness() -> impl Responder {
    let h = HealthEndpointResponseModel::new();
    HttpResponse::Ok().json(h)
}
