pub mod models;
pub mod schema;

use crate::models::Order;
use crate::schema::shop_module_order::dsl::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use reqwest::header::{HeaderMap, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use rocket::get;
use rocket::serde::json::Json;
use serde_json::json;
use std::{env, fs};
use std::path::Path;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[get("/orders")]
pub fn list_orders() -> Json<Vec<Order>> {
    let connection = &mut establish_connection();
    let results = shop_module_order
        .filter(esim.eq(true))
        .limit(10)
        .select(Order::as_select())
        .order_by(created.desc())
        .load(connection)
        .expect("Error loading orders");
    Json(results)
}

#[get("/logs/<file_name>")]
pub fn logs(file_name: &str) -> String {
    if !Path::new(file_name).exists() {
        "Invalid file name".to_string()
    } else {
        let content = fs::read_to_string(file_name);
        content.unwrap()
    }
}

pub fn send_zepto_email(send_to: &str, uid: &str, host: &str) {
    let post_body = json!({
        "template_key": env::var("QR_ORDER_KEY").expect("Invalid QR_ORDER_KEY").as_str(),
        "from": {"address": "listen@xplori.world", "name": "Xplori"},
        "to": send_to,
        "merge_info": {"uid": uid, "host": host}
    });

    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, "application/json".parse().unwrap());
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(
        AUTHORIZATION,
        env::var("ZEPTO_MAIL_API_KEY")
            .expect("Invalid ZEPTO_MAIL_API_KEY")
            .parse()
            .unwrap(),
    );
    // let client = reqwest::blocking::Client::builder().default_headers(headers).build().unwrap();
    let client = reqwest::blocking::Client::new();
    let res = client
        .post("https://api.zeptomail.com/v1.1/email/template")
        .headers(headers)
        .json(&post_body)
        .send()
        .unwrap();
    println!("{:?}", res);
}
