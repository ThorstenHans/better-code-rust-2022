use actix_web::{
    get, post,
    web::{Json, Path},
    HttpResponse, Responder,
};
use diesel::prelude::*;

use crate::db;
use crate::schema::products::dsl::*;
use log::debug;

use super::models::{CreateProductModel, Product};

#[get("")]
pub async fn all() -> impl Responder {
    debug!("Receive all products invoked");
    let connection = &mut db::establish_connection();
    let results = products
        .limit(5)
        .load::<Product>(connection)
        .expect("Error loading products");
    HttpResponse::Ok().json(results)
}

#[get("/{id}")]
pub async fn get_by_id(_id: Path<i32>) -> impl Responder {
    let connection = &mut db::establish_connection();
    match products
        .filter(id.eq(_id.into_inner()))
        .first::<Product>(connection)
    {
        Ok(product) => HttpResponse::Ok().json(product),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[post("")]
pub(crate) async fn create(product: Json<CreateProductModel>) -> impl Responder {
    use crate::schema::products;

    let model = product.into_inner();
    let connection = &mut db::establish_connection();
    match diesel::insert_into(products::table)
        .values(&model)
        .get_result::<Product>(connection)
    {
        Ok(product) => HttpResponse::Created().json(product),
        Err(_) => HttpResponse::InternalServerError().body("Error while creating product"),
    }
}
