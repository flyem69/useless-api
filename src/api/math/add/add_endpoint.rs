use rocket::get;

#[get("/math/add?<values>")]
pub fn get_addition(values: Vec<i32>) -> String {
    let mut result = 0;
    for value in values {
        result += value;
    }
    result.to_string()
}
