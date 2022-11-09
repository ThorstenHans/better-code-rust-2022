use diesel::prelude::*;
use crate::schema::products;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Insertable)]
#[diesel(table_name = products)]
pub struct CreateProductModel {
    name: String,
    price: f64,
}

#[derive(Queryable, Serialize)]
pub struct Product {
    id: i32,
    name: String,
    price: f64,
}
