use actix_web::HttpResponse;
use actix_web::{Responder, body::BoxBody,
     http::header::ContentType,};
use serde::Deserialize;
use serde::Serialize;

use crate::schema::users;
use crate::schema::statistics;
use crate::schema::objects;

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
}

#[derive(Queryable, QueryableByName, Debug, AsChangeset, Serialize, Deserialize)]
#[table_name = "users"]
pub struct User{
    pub id: i32,
    pub username: String,
}

#[derive(Insertable)]
#[table_name = "statistics"]
pub struct NewStatistic<'a> {
    pub name: &'a str,
    pub user_id: i32,
}

#[derive(Queryable,QueryableByName , Debug, AsChangeset, Serialize, Deserialize)]
#[table_name = "statistics"]
pub struct Statistic{
    pub id: i32,
    pub name: String,
    pub user_id : i32,
}
pub fn build_statistic(id: i32, name: String) -> Statistic {
    Statistic { id: id, name: name, user_id: -1 }
}
impl Responder for Statistic {
    type Body = BoxBody;
    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}
#[derive(Insertable)]
#[table_name = "objects"]
pub struct NewObject<'a> {
    pub name: &'a str,
    pub statistic_id: i32,
    pub counter: i32,
}

#[derive(Queryable,QueryableByName , Debug, AsChangeset, Serialize, Deserialize)]
#[table_name = "objects"]
pub struct Object{
    pub id: i32,
    pub name: String,
    pub counter: i32,
    pub statistic_id: i32,
}
