use rocket::get;

#[get("/math/add?<values>")]
pub fn get_addition(values: Vec<i32>) -> String {
    let mut result: i32 = 0;
    for value in values {
        result += value;
    }
    result.to_string()
}

#[get("/math/subtract/<minuend>/<subtrahend>")]
pub fn get_subtraction(minuend: i32, subtrahend: i32) -> String {
    (minuend - subtrahend).to_string()
}
