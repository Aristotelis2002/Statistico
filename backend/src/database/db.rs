use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection{
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
      .expect("DATABASE URL must be set");
      //make sure your path is correctly set in the .env file
    SqliteConnection::establish(&database_url)
      .expect(&format!("Failed to connect to {}",database_url))
}