use diesel::prelude::*;
use genesis::{list_orders, logs};
use rocket::{launch, routes, Build, Rocket};

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![list_orders, logs])
}
