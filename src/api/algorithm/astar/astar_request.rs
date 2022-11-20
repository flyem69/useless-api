use async_trait::async_trait;
use rocket::{Data, Request};
use rocket::data::{ByteUnit, FromData, Outcome};
use rocket::http::{ContentType, Status};
use rocket::serde::Deserialize;
use rocket::tokio::io::AsyncReadExt;

#[derive(Deserialize)]
pub struct AstarRequest {
    pub value: String,
}

#[async_trait]
impl<'a> FromData<'a> for AstarRequest {
    type Error = String;

    async fn from_data(request: &'a Request<'_>, request_data: Data<'a>) -> Outcome<'a, Self, String> {
        let content_type = ContentType::new("application", "json");
        if request.content_type() != Some(&content_type) {
            return Outcome::Forward(request_data); // todo return problem response
        }

        let mut request_data_stream = request_data.open(ByteUnit::Kilobyte(256));
        let mut request_body = String::new();
        let request_body_result = match request_data_stream.read_to_string(&mut request_body).await {
            Ok(_) => Ok(request_body),
            Err(_) => Err(()),
        };
        if let Err(()) = request_body_result {
            return Outcome::Failure((Status::BadRequest, "dupa1".to_string())); // todo name it
        }
        let request_body = request_body_result.unwrap();

        let astar_request_result: Result<AstarRequest, ()> = match serde_json::from_str(&*request_body) {
            Ok(result) => Ok(result),
            Err(_) => Err(()),
        };
        if let Err(()) = astar_request_result {
            return Outcome::Failure((Status::BadRequest, "dupa2".to_string())); // todo name it
        }
        let astar_request = astar_request_result.unwrap();

        Outcome::Success(astar_request)
    }
}
