use rocket::post;

use crate::api::algorithm::astar::astar_request::AstarRequest;

#[post("/algorithms/astar", data = "<astar_request>")]
pub fn post_astar(astar_request: AstarRequest) -> String {
    astar_request.value.to_string()
}
