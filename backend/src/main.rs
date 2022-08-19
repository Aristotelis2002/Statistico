#[macro_use]
extern crate diesel;
extern crate dotenvy;

pub mod schema;
pub mod database;
//mod database;
mod ops;

fn main() {
    println!("Hello, world!");
}
