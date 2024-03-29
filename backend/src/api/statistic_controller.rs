use actix_web::{get, http::header::ContentType, post, web, HttpResponse, Responder};
use serde::Deserialize;
use crate::database::models::build_statistic;

use crate::{
    database::models::{NewStatistic, Statistic},
    ops::statistic_ops::*,
};

#[get("/stats/{user_id}")] 
pub async fn get_all_statistics(info: web::Path<i32>) -> impl Responder {
    let user_id: i32 = info.into_inner();
    let all_statistics: Option<Vec<Statistic>> = show_statistic_by_user_id(user_id);
    match all_statistics {
        None => HttpResponse::BadRequest().json(false),
        Some(statistics) => HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&statistics).unwrap_or_default())
    }
    
}
#[derive(Deserialize)]
pub struct StatisticInfo {
    name: String,
    user_id: i32,
}
#[post("/stats/add/")]
pub async fn add_new_statistic(info: web::Json<StatisticInfo>) -> impl Responder {
    let info_longer = info.into_inner(); 
    let statistic_new: NewStatistic = NewStatistic {
        name: (info_longer.name.as_str()),
        user_id: (info_longer.user_id),
    };

    if create_statistic(statistic_new) {
        HttpResponse::Ok().json(true)
    } else {
        HttpResponse::BadRequest().json(false)
    }
} //curl "http://localhost:8080/stats/add/" -X "POST" -H "Content-Type: application/json" -d '{"name": "movies", "user_id": 1}'

#[post("stats/delete/")]
pub async fn delete_statistic_service(info: web::Json<i32>) -> impl Responder {
    if delete_statistic(info.into_inner()) {
        HttpResponse::Ok().json(true)
    } else {
        HttpResponse::BadRequest().json(false)
    }
} //curl "http://localhost:8080/stats/delete/" -X POST -H "Content-Type: application/json" -d '???

#[derive(Deserialize)]
pub struct StatisticInfoUpdate {
    id: i32,
    name: String,
}
#[post("stats/rename/")]
pub async fn rename_statistic(info: web::Json<StatisticInfoUpdate>) -> impl Responder {
    let info_longer = info.into_inner();
    let statistic_new: Statistic = build_statistic(info_longer.id, info_longer.name);

    if update_statistic(statistic_new) {
        HttpResponse::Ok().json(true)
    } else {
        HttpResponse::BadRequest().json(false)
    }
} //curl "http://localhost:8080/stats/rename/" -X "POST" -H "Content-Type: application/json" -d '{"id": 2, "name": "movies"}
