use actix_web::{
    body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, Responder,
    HttpServer, App, get, web
};
use serde::Serialize;

use crate::{ops::statistic_ops::*, database::models::Statistic};

#[get("/stats/{user_id}")] //change to json_get
pub async fn get_all_statistics(info: web::Path<i32>) -> impl Responder {
    let user_id: i32 = info.into_inner();
    let all_statistics:Vec<Statistic> = show_statistic_by_user_id(user_id);
    
   HttpResponse::Ok()
       .content_type(ContentType::json())
       .body(serde_json::to_string(&all_statistics).unwrap())
}
