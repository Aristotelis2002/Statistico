use crate::database::db::establish_connection;
use crate::database::models::{NewUser, User};
use diesel::{prelude::*, connection};
use diesel::dsl::*;
pub fn create_user(user: NewUser) {
    use crate::schema::users;
    let connection = establish_connection();
    diesel::insert_into(users::table)
        .values(&user)
        .execute(&connection)
        .expect("Error adding new user");

}
pub fn delete_user(user_id: i32) {
    use crate::schema::users::dsl::*;
    let connection = establish_connection();
    diesel::delete(users.find(user_id))
        .execute(&connection)
        .expect("Failed to delete user");
}
pub fn show_by_user_id(user_id: i32) -> User {
    use crate::schema::users;
    let connection = establish_connection();
    users::table.find(user_id).get_result::<User>(&connection)
    .ok()
    .expect("Failed to find user")
}
//TODO 
#[allow(dead_code)]
pub fn rename_user(){
    todo!()
}