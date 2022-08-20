use actix_web::{
    body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, Responder,
    HttpServer, App, get, web, post
};
use serde::{Serialize, Deserialize};


use crate::{ops::statistic_ops::*, database::models::{Statistic, NewStatistic}};

#[get("/stats/{user_id}")] //maybe change to json_get
pub async fn get_all_statistics(info: web::Path<i32>) -> impl Responder {
    let user_id: i32 = info.into_inner();
    let all_statistics:Vec<Statistic> = show_statistic_by_user_id(user_id);
    
   HttpResponse::Ok()
       .content_type(ContentType::json())
       .body(serde_json::to_string(&all_statistics).unwrap())
}
#[derive(Deserialize)]
pub struct StatisticInfo{
    name: String,
    user_id: i32,   
}
#[post("/stats/add/")]
pub async fn add_new_statistic(info: web::Json<StatisticInfo>) -> impl Responder{
    let info_longer = info.into_inner();//valid Json?
    let statistic_new:NewStatistic = NewStatistic {
         name: (info_longer.name.as_str()), 
         user_id: (info_longer.user_id) };
    if create_statistic(statistic_new) {
        HttpResponse::Ok()
            .json(true)
    }else{
        HttpResponse::BadRequest()
            .json(false)    
    }
}

#[post("stats/delete/{stat_id}")]
pub async fn delete_statistic_service(info: web::Path<i32>) -> impl Responder{
    if delete_statistic(info.into_inner()) { //into_inner() good?
        HttpResponse::Ok()
            .json(true)
    }else{
        HttpResponse::BadRequest()
            .json(false)    
    }
}

#[derive(Deserialize)]
pub struct StatisticInfoUpdate{
    statistic_id: i32,   
    name: String,
}
#[post("stats/rename/")]
pub async fn rename_statistic(info: web::Json<StatisticInfoUpdate>) -> impl Responder{
    let info_longer = info.into_inner();
    let statistic_new: Statistic = Statistic { 
        id: info_longer.statistic_id, 
        name: info_longer.name, 
        user_id: -1 
    };
    if update_statistic(statistic_new) {
        HttpResponse::Ok()
            .json(true)
    }else{
        HttpResponse::BadRequest()
            .json(false)    
    }
}