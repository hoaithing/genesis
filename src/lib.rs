#[macro_use]
extern crate diesel;
extern crate dotenv;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod models;

pub mod show {
    pub fn show_name(name: &String) {
        println!("{}", name);
    }
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
