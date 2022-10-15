#[macro_use]
extern crate rocket;

use rocket::{Build, get, launch};

use api::math_endpoint;

mod api;

#[get("/")]
fn index() -> &'static str {
    "Hello world"
}

#[launch]
fn rocket() -> rocket::Rocket<Build> {
    rocket::build().mount("/", routes![
        index,
        math_endpoint::get_addition,
        math_endpoint::get_subtraction
    ])
}
