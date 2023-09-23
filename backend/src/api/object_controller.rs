use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;

use crate::{
    database::models::NewObject,
    ops::object_ops::*,
};

#[get("/load/stat/{statistic_id}")]
pub async fn get_all_objects(info: web::Path<i32>) -> impl Responder {
    let statistic_id: i32 = info.into_inner();
    match show_objects_by_statistic_id(statistic_id) {
        None => HttpResponse::BadRequest().json(false),
        Some(objects) => HttpResponse::Ok().json(objects),
    }
}
#[derive(Deserialize)]
pub struct ObjectInfoAdd {
    name: String,
    statistic_id: i32,
    counter: i32,
}
#[post("/object/add/")] 
pub async fn add_new_object(info: web::Json<ObjectInfoAdd>) -> impl Responder {
    let info_longer = info.into_inner();
    let object_new: NewObject = NewObject {
        name: &info_longer.name,
        statistic_id: info_longer.statistic_id,
        counter: info_longer.counter,
    };
    if create_object(object_new) {
        HttpResponse::Ok().json(true)
    } else {
        HttpResponse::BadRequest().json(false)
    }
}
#[derive(Deserialize)]
pub struct ObjectInfoCounter {
    id: i32,
    counter: i32,
}
#[post("/object/counter/update/")]
pub async fn update_counter_object(info: web::Json<ObjectInfoCounter>) -> impl Responder {
    let info_longer = info.into_inner();
    let object_id = info_longer.id;
    let value_new = info_longer.counter;
    let res = update_counter(object_id, value_new);
    match res {
        None => HttpResponse::BadRequest().json(false),
        Some(0) => HttpResponse::BadRequest().json(false),
        Some(_) => HttpResponse::Ok().json(true),
    }
}
#[post("/object/delete/")]
pub async fn delete_object_service(info: web::Json<i32>) -> impl Responder {
    let object_id = info.into_inner();
    let res = delete_object(object_id);
    if res {
        HttpResponse::Ok().json(true)
    } else {
        HttpResponse::BadRequest().json(false)
    }
}
#[derive(Deserialize)]
pub struct ObjectInfo {
    id: i32,
    name: String,
}
#[post("/object/rename/")]
pub async fn rename_object(info: web::Json<ObjectInfo>) -> impl Responder {
    let object = info.into_inner();
    let res = update_object_name(object.id, object.name);
    if res {
        HttpResponse::Ok().json(true)
    } else {
        HttpResponse::BadRequest().json(false)
    }
}
//curl "http://localhost:8080/object/rename/" -X "POST" -H "Content-Type: application/json" -d '{"name": "little book", "id": 3}'
