use crate::utils::jwt::jwt::is_valid;
use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::{request, Outcome, Request};

#[derive(Debug)]
pub struct ApiKey {
    pub key: String,
}

#[derive(Debug)]
pub enum ApiKeyError {
    BadCount,
    Missing,
    Invalid,
}

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = ApiKeyError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        // unimplemented!()
        let keys: Vec<_> = request.headers().get("Authorization").collect();

        match keys.len() {
            0 => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
            1 if is_valid(keys[0]) => Outcome::Success(ApiKey {
                key: keys[0].to_string(),
            }),
            1 => Outcome::Failure((Status::Forbidden, ApiKeyError::Invalid)),
            _ => Outcome::Failure((Status::Forbidden, ApiKeyError::BadCount)),
        }
    }
}