use crate::database::db::establish_connection;
use crate::database::models::{NewStatistic, Statistic};
use crate::schema::statistics;
use diesel::dsl::*;
use diesel::prelude::*;

pub fn create_statistic(new_stat: NewStatistic) -> bool {
    let connection = establish_connection();
    diesel::insert_into(statistics::table)
        .values(&new_stat)
        .execute(&connection)
        .is_ok()
}
pub fn delete_statistic(statistic_id: i32) -> bool {
    use crate::schema::statistics::dsl::*;
    let connection = establish_connection();
    diesel::delete(statistics.find(statistic_id))
        .execute(&connection)
        .is_ok()
}
pub fn show_statistic_by_id(statistic_id: i32) -> Option<Statistic> {
    let connection = establish_connection();
    statistics::table
        .find(statistic_id)
        .get_result::<Statistic>(&connection)
        .ok()
}
pub fn show_statistic_by_user_id(user_id: i32) -> Option<Vec<Statistic>> {
    let connection = establish_connection();
    sql_query(format!(
        "SELECT id, name, user_id
                 FROM statistics WHERE statistics.user_id = {}",
        user_id
    ))
    .load(&connection)
    .ok()
}
pub fn update_statistic(statistic_entity: Statistic) -> bool {
    let connection = establish_connection();
    diesel::update(statistics::table.find(statistic_entity.id))
        .set(statistics::name.eq(statistic_entity.name))
        .execute(&connection)
        .is_ok()
}
