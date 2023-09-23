use crate::database::db::establish_connection;
use crate::database::models::{NewObject, Object};
use crate::schema::objects;
use diesel::dsl::*;
use diesel::prelude::*;

pub fn create_object(new_object: NewObject) -> bool {
    let connection = establish_connection();
    diesel::insert_into(objects::table)
        .values(&new_object)
        .execute(&connection)
        .is_ok()
}
pub fn delete_object(object_id: i32) -> bool {
    use crate::schema::objects::dsl::*;
    let connection = establish_connection();
    diesel::delete(objects.find(object_id))
        .execute(&connection)
        .is_ok()
}
pub fn show_object_by_id(object_id: i32) -> Option<Object> {
    let connection = establish_connection();

    objects::table
        .find(object_id)
        .get_result::<Object>(&connection)
        .ok()
}
pub fn show_objects_by_statistic_id(statistic_id: i32) -> Option<Vec<Object>> {
    let connection = establish_connection();
    sql_query(format!(
        "SELECT *
                 FROM objects WHERE objects.statistic_id = {}",
        statistic_id
    ))
    .load(&connection)
    .ok()
}
pub fn update_object(object_entity: Object) -> bool {
    let connection = establish_connection();
    if diesel::update(objects::table.find(object_entity.id))
        .set(objects::name.eq(object_entity.name))
        .execute(&connection)
        .is_err() {
            return false;
        }

    return update_counter(object_entity.id, object_entity.counter).is_some();
}
pub fn update_object_name(id_new: i32, name_new: String) -> bool {
    let connection = establish_connection();
    diesel::update(objects::table.find(id_new))
        .set(objects::name.eq(name_new))
        .execute(&connection)
        .is_ok()
}

pub fn update_counter(object_id: i32, counter_value: i32) -> Option<usize> {
    let connection = establish_connection();
    diesel::update(objects::table.find(object_id))
        .set(objects::counter.eq(counter_value))
        .execute(&connection)
        .ok()
}
