use rocket::get;

#[get("/math/subtract?<minuend>&<subtrahend>")]
pub fn get_subtraction(minuend: i32, subtrahend: i32) -> String {
    (minuend - subtrahend).to_string()
}
