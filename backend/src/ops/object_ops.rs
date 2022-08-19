use crate::database::db::establish_connection;
use crate::database::models::{NewObject, Object};
use crate::schema::objects;
use diesel::{prelude::*, connection};
use diesel::dsl::*;


pub fn create_object(new_object: NewObject){
    let connection = establish_connection();
    diesel::insert_into(objects::table)
        .values(&new_object)
        .execute(&connection)
        .expect("Failed creating a new object");
}
pub fn delete_object(object_id : i32) {
    use crate::schema::objects::dsl::*;
    let connection = establish_connection();
    diesel::delete(objects.find(object_id))
        .execute(&connection)
        .expect("Failed to delete object");
}
pub fn show_object_by_id(object_id : i32) -> Object{
    let connection = establish_connection();
    objects::table.find(object_id)
        .get_result::<Object>(&connection)
        .ok()
        .expect("Failed to find object")
}
pub fn show_objects_by_statistic_id(statistic_id: i32) -> Vec<Object>{
    let connection = establish_connection();
    sql_query(format!("SELECT *
                 FROM statistics WHERE objects.statistic_id = {}", statistic_id))
        .load(&connection)
        .expect("Find object by statistic id query failed")
}
pub fn update_object(object_entity: Object) {
    let connection = establish_connection();
    diesel::update(objects::table
        .find(object_entity.id))
        .set(objects::name.eq(object_entity.name))
        .execute(&connection)
        .expect("Couldn't update object name");

    update_counter(object_entity.id, object_entity.counter);
    // diesel::update(objects::table
    //     .find(object_entity.id))
    //     .set(objects::counter.eq(object_entity.counter))
    //     .execute(&connection)
    //     .expect("Couldn't update object counter");
}
pub fn update_counter(object_id : i32, counter_value: i32){
    let connection = establish_connection();
    diesel::update(objects::table
        .find(object_id))
        .set(objects::counter.eq(counter_value))
        .execute(&connection)
        .expect("Couldn't update object counter");
}