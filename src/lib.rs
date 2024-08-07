pub mod models;
pub mod schema;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use rocket::get;
use rocket::serde::json::Json;

use crate::models::Order;
use crate::schema::shop_module_order::dsl::*;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[get("/")]
pub fn list_orders() -> Json<Vec<Order>>{
    let connection = &mut establish_connection();
    let results = shop_module_order.filter(esim.eq(true))
        .limit(10)
        .select(Order::as_select() )
        .order_by(created.desc())
        .load(connection)
        .expect("Error loading orders");
    Json(results)
}
