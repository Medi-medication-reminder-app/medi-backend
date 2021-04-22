use rocket_contrib::json::{Json, JsonError};

use crate::utils::response::*;
use crate::DbConn;
use crate::utils::jwt::{apikey::ApiKey, claim::get_claim};
use crate::models::api::today::*;

#[get("/")]
fn read(conn: DbConn, key: ApiKey) -> Result<ApiResponse, ApiError> {
    let claim = match get_claim(key.key.as_str()) {
        Some(c) => c,
        None => return Err(fail(401, String::from("Unauthorized"), String::from("Invalid token"))),
    };

    let result = TodayTreatment::read(claim.usr, &conn);
    match result {
        Ok(r) => Ok(success(json!(r))),
        Err(e) => Err(db_error(e)),
    }
}

#[put("/", data = "<data>")]
fn update(
    data: Result<Json<TodayTakeForm>, JsonError>,
    conn: DbConn,
    key: ApiKey
) -> Result<ApiResponse, ApiError> {
    let claim = match get_claim(key.key.as_str()) {
        Some(c) => c,
        None => return Err(fail(401, String::from("Unauthorized"), String::from("Invalid token"))),
    };

    match data {
        Ok(d) => {
            let new = TodayTakeForm {
                ..d.into_inner()
            };
            let result = TodayTakeForm::create(
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
    // routes![read]
    routes![read, update]
}