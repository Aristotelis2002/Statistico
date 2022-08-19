use crate::database::db::establish_connection;
use crate::database::models::{NewStatistic, Statistic};
use crate::schema::statistics;
use diesel::{prelude::*, connection};
use diesel::dsl::*;

pub fn create_statistic(new_stat: NewStatistic){
    let connection = establish_connection();
    diesel::insert_into(statistics::table)
        .values(&new_stat)
        .execute(&connection)
        .expect("Failed creating a new statistic");
}
pub fn delete_statistic(statistic_id : i32) {
    use crate::schema::statistics::dsl::*;
    let connection = establish_connection();
    diesel::delete(statistics.find(statistic_id))
        .execute(&connection)
        .expect("Failed to delete statistic");
}
pub fn show_statistic_by_id(statistic_id : i32) -> Statistic{
    let connection = establish_connection();
    statistics::table.find(statistic_id)
    .get_result::<Statistic>(&connection)
    .ok()
    .expect("Failed to find statistic")
}
pub fn show_statistic_by_user_id(user_id:i32) -> Vec<Statistic>{
    let connection = establish_connection();
    sql_query(format!("SELECT id, name, user_id
                 FROM statistics WHERE statistics.user_id = {}",user_id))
        .load(&connection)
        .expect("Find statistics by user id query failed")
}
pub fn update_statistic(statistic_entity: Statistic) {
    let connection = establish_connection();
    let _ = diesel::update(statistics::table
        .find(statistic_entity.id))
        .set(statistics::name.eq(statistic_entity.name))
        .execute(&connection)
        .expect("Failed to update statistic");

}