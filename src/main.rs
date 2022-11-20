#[macro_use]
extern crate rocket;

use rocket::{Build, get, launch};

mod api;

#[get("/")]
fn index() -> &'static str {
    "This is useless API. Enjoy!"
}

#[launch]
fn rocket() -> rocket::Rocket<Build> {
    rocket::build().mount(
        "/",
        routes![
            index,
            api::algorithm::astar::astar_endpoint::post_astar,
            api::math::add::add_endpoint::get_addition,
            api::math::subtract::subtract_endpoint::get_subtraction,
        ],
    )
}
