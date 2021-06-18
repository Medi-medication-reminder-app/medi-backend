use rocket_contrib::json::{Json, JsonError};
// use chrono::Utc;

use crate::utils::response::*;
use crate::DbConn;
use crate::utils::jwt::{apikey::ApiKey, claim::get_claim};
use crate::models::api::calendar::*;

#[get("/thisweek")]
fn read(conn: DbConn, key: ApiKey) -> Result<ApiResponse, ApiError> {
    let claim = match get_claim(key.key.as_str()) {
        Some(c) => c,
        None => return Err(fail(401, String::from("Unauthorized"), String::from("Invalid token"))),
    };

    let result = CalendarTreatment::read_thisweek(
        claim.usr,
        &conn
    );
    match result {
        Ok(r) => Ok(success(json!(r))),
        Err(e) => Err(db_error(e)),
    }
}

#[post("/between", data = "<data>")]
fn read_between(data: Result<Json<DateRangeForm>, JsonError>, conn: DbConn, key: ApiKey) -> Result<ApiResponse, ApiError> {
    let claim = match get_claim(key.key.as_str()) {
        Some(c) => c,
        None => return Err(fail(401, String::from("Unauthorized"), String::from("Invalid token"))),
    };

    match data {
        Ok(d) => {
            let range = DateRangeForm {
                ..d.into_inner()
            };
            let result = CalendarTreatment::read_between(
                range.start,
                range.end,
                claim.usr,
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
    routes![read, read_between]
}