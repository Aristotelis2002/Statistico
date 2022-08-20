use actix_web::{
    post, http::header::ContentType, HttpRequest, HttpResponse, Responder,
    HttpServer, App, get, web
};
use serde::{Serialize, Deserialize};

use crate::{ops::object_ops::*, database::models::Object};

#[get("/load/stat/{statistic_id}")]
pub async fn get_all_objects(info: web::Path<i32>) -> impl Responder{
    let statistic_id: i32 = info.into_inner();
    let get_all_objects: Vec<Object> = show_objects_by_statistic_id(statistic_id);
    HttpResponse::Ok()
        .json(get_all_objects)
}
#[post("/stat/counter/{object_id}/{value_new}")]
pub async fn update_counter_object(info: web::Path<(i32,i32)>) -> impl Responder{
    let (object_id, value_new) = info.into_inner();
    let res = update_counter(object_id, value_new);
    if res.is_none(){
      return  HttpResponse::BadRequest()
                .json(false);
    }
    if res.unwrap() != 0{
        HttpResponse::Ok()
            .json(true)
            
    }else{
        HttpResponse::BadRequest()
            .json(false)
    }
}
#[post("/object/delete/{object_id}")]
pub async fn delete_object_service(info: web::Path<i32>) -> impl Responder{
    let object_id = info.into_inner();
    let res = delete_object(object_id);
    if res {
        HttpResponse::Ok()
            .json(true)
    }else {
        HttpResponse::BadRequest()
            .json(false)
    }
}
#[derive(Deserialize)]
pub struct ObjectInfo{
    name: String,
    id: i32,
}
#[post("/object/rename/")]
pub async fn rename_object(info: web::Json<ObjectInfo>) -> impl Responder{
    let object  = info.into_inner();
    let res = update_object_name(object.id, object.name);
    if res {
        HttpResponse::Ok()
            .json(true)
    }else {
        HttpResponse::BadRequest()
            .json(false)
    }
}
//curl "http://localhost:8080/object/rename/" -X "POST" -H "Content-Type: application/json" -d '{"name": "little book", "id": 3}'
