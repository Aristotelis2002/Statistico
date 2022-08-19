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

#[derive(Queryable, Debug, AsChangeset, Serialize, Deserialize)]
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