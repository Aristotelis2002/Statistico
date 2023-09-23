#[macro_use]
extern crate diesel;
extern crate dotenvy;

pub mod database;
pub mod ops;
pub mod schema;
mod api;
mod tests;
use actix_web::{middleware::Logger, App, HttpServer};
use api::statistic_controller::*;
use actix_cors::Cors;
use api::chart_controller::*;
use api::object_controller::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    HttpServer::new(|| {
        let logger: Logger = Logger::default();
        let cors = Cors::permissive();
        App::new()
            .wrap(logger)
            .wrap(cors)
            .service(get_all_statistics)
            .service(get_all_objects)
            .service(update_counter_object)
            .service(delete_object_service)
            .service(rename_object)
            .service(add_new_object)
            .service(add_new_statistic)
            .service(delete_statistic_service)
            .service(rename_statistic)
            .service(make_chart)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
