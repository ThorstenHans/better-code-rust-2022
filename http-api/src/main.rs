use crate::healthz::{liveness, readiness, HEALTHZ_ROUTE_PREFIX};
use crate::products::{all, create, get_by_id, PRODUCTS_ROUTE_PREFIX};
use actix_web::{middleware, App, HttpServer, Scope};

mod db;
mod healthz;
mod products;
mod schema;

const DEFAULT_ADDR: &str = "0.0.0.0";
const DEFAULT_PORT: u16 = 8080;

fn init_log() {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_log();
    let address = get_address();
    HttpServer::new(|| {
        let healthz = Scope::new(HEALTHZ_ROUTE_PREFIX)
            .service(liveness)
            .service(readiness);

        let products = Scope::new(PRODUCTS_ROUTE_PREFIX)
            .service(all)
            .service(get_by_id)
            .service(create);

        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::new(
                middleware::TrailingSlash::Trim,
            ))
            .wrap(middleware::Compress::default())
            .service(products)
            .service(healthz)
    })
    .bind(address)?
    .run()
    .await
}

fn get_address() -> (String, u16) {
    let address = std::env::var("ADDRESS").ok();

    let port = std::env::var("PORT").ok().and_then(|p| p.parse().ok());

    match address.zip(port) {
        Some((address, port)) => (address, port),
        None => (DEFAULT_ADDR.to_owned(), DEFAULT_PORT),
    }
}
