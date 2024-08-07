use diesel::prelude::*;
use rocket::{Build, launch, Rocket, routes};
use genesis::list_orders;

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![list_orders])
}
