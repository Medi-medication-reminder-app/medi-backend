use rocket_contrib::json::{Json, JsonError};

use crate::utils::response::*;
use crate::DbConn;
use crate::utils::jwt::{apikey::ApiKey, claim::get_claim};
use crate::models::api::journal::JournalForm;
use crate::models::api::calendar::DateRangeForm;
use crate::models::database::feeling::Feeling;

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
            let result = JournalForm::read_between(
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

#[get("/feeling")]
fn read_feelings(conn: DbConn, key: ApiKey) -> Result<ApiResponse, ApiError> {
    let _claim = match get_claim(key.key.as_str()) {
        Some(c) => c,
        None => return Err(fail(401, String::from("Unauthorized"), String::from("Invalid token"))),
    };

    let result = Feeling::read(&conn);
    match result {
        Ok(r) => Ok(success(json!(r.into_iter().map(|f| f.feeling_name).collect::<Vec<String>>()))),
        Err(e) => Err(db_error(e)),
    }
}

pub fn routes() -> Vec<rocket::Route> {
    routes![read, create, read_between, read_feelings]
}