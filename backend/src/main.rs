use database::models::{NewUser, Statistic, NewStatistic, NewObject};

#[macro_use]
extern crate diesel;
extern crate dotenvy;

pub mod schema;
pub mod database;
pub mod ops;
//use crate::ops::user_ops::*;
//use crate::ops::statistic_ops::*;
//use crate::{ops::object_ops::*, database::models::Object};
mod api;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, middleware::Logger};
use api::statistic_controller::*;
use api::user_controller::*;
use api::object_controller::*; 
use actix_cors::Cors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG","debug");
    std::env::set_var("RUST_BACKTRACE","1");
    env_logger::init();
    HttpServer::new(|| {
        let logger: Logger = Logger::default();
        let cors = Cors::permissive();
        App::new()
        .wrap(logger)
        .wrap(cors)
        .service(get_all_statistics)
        .service(get_all_objects)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
