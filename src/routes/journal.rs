use rocket_contrib::json::{Json, JsonError};

use crate::utils::response::*;
use crate::DbConn;
use crate::utils::jwt::{apikey::ApiKey, claim::get_claim};
use crate::models::api::journal::JournalForm;

#[get("/")]
fn read(conn: DbConn, key: ApiKey) -> Result<ApiResponse, ApiError> {
    let claim = match get_claim(key.key.as_str()) {
        Some(c) => c,
        None => return Err(fail(401, String::from("Unauthorized"), String::from("Invalid token"))),
    };

    let result = JournalForm::read(claim.usr, &conn);
    match result {
        Ok(r) => Ok(success(json!(r))),
        Err(e) => Err(db_error(e)),
    }
}

#[post("/", data = "<data>")]
fn create(
    data: Result<Json<JournalForm>, JsonError>,
    conn: DbConn,
    key: ApiKey
) -> Result<ApiResponse, ApiError> {
    let claim = match get_claim(key.key.as_str()) {
        Some(c) => c,
        None => return Err(fail(401, String::from("Unauthorized"), String::from("Invalid token"))),
    };

    match data {
        Ok(d) => {
            let new = JournalForm {
                ..d.into_inner()
            };
            let result = JournalForm::create(
                claim.usr,
                new,
                &conn
            );
            match result {
                Ok(r) => Ok(success(json!(r))),
                Err(e) => Err(db_error(e)),
            }
        }
        Err(e) => Err(json_error(e)),
    }
}

pub fn routes() -> Vec<rocket::Route> {
    routes![read, create]
}