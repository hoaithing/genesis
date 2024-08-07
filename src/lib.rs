pub mod models;
pub mod schema;

use std::collections::HashMap;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use reqwest::header::{ACCEPT, HeaderMap, CONTENT_TYPE, AUTHORIZATION};
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


pub fn send_zepto_email(send_to: &str, uid: &str) {
    let mut post_body = HashMap::new();
    post_body.insert("template_key", env::var("QR_ORDER_KEY")
        .expect("Invalid QR_ORDER_KEY")
        .as_str());
    let mut from = HashMap::new();

    from.insert("address", "noreply@xplori.world");
    from.insert("name", "Xplori");

    post_body.insert("from", from);

    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, "application/json".parse().unwrap());
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(AUTHORIZATION, env::var("ZEPTO_MAIL_API_KEY")
        .expect("Invalid ZEPTO_MAIL_API_KEY")
        .parse()
        .unwrap());
    // let client = reqwest::blocking::Client::builder().default_headers(headers).build().unwrap();
    let client = reqwest::blocking::Client::new();
    let res = client.post("https://api.zeptomail.com/v1.1/email/template")
        .headers(headers)
        .send()
        .unwrap()
        .json()
        .unwrap();
    println!("{}", res);
}