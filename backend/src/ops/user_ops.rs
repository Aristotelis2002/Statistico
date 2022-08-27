use crate::database::db::establish_connection;
use crate::database::models::{NewUser, User};
use diesel::{prelude::*, connection};
use diesel::dsl::*;
pub fn create_user(user: NewUser) -> Result<i32, diesel::result::Error> {
    use crate::schema::users;
    let connection = establish_connection();
    diesel::insert_into(users::table)
        .values(&user)
        .execute(&connection)
        .expect("Error adding new user");
        no_arg_sql_function!(
            last_insert_rowid,
            diesel::sql_types::Integer,
            "Represents the SQL last_insert_row() function"
        );
        diesel::select(last_insert_rowid)
     .get_result::<i32>(&connection)
}
pub fn delete_user(user_id: i32) {
    use crate::schema::users::dsl::*;
    let connection = establish_connection();
    diesel::delete(users.find(user_id))
        .execute(&connection)
        .expect("Failed to delete user");
}
pub fn show_by_user_id(user_id: i32) -> Option<User> {
    use crate::schema::users;
    let connection = establish_connection();
    users::table.find(user_id).get_result::<User>(&connection)
    .ok()
    
}
//TODO 
#[allow(dead_code)]
pub fn rename_user(){
    todo!()
}