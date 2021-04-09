use rocket_contrib::json::{Json, JsonError};

use crate::utils::response::*;
use crate::DbConn;
use crate::utils::jwt::{apikey::ApiKey, claim::get_claim};
use crate::models::api::treatment::*;

#[get("/")]
fn read(conn: DbConn, key: ApiKey) -> Result<ApiResponse, ApiError> {
    let claim = match get_claim(key.key.as_str()) {
        Some(c) => c,
        None => return Err(fail(401, String::from("Unauthorized"), String::from("Invalid token"))),
    };

    let result = TreatmentForm::read(claim.usr, &conn);
    match result {
        Ok(r) => Ok(success(json!(r))),
        Err(e) => Err(db_error(e)),
    }
}

#[post("/", data = "<data>")]
fn create(
    data: Result<Json<TreatmentForm>, JsonError>,
    conn: DbConn, 
    key: ApiKey
) -> Result<ApiResponse, ApiError> {
    let claim = match get_claim(key.key.as_str()) {
        Some(c) => c,
        None => return Err(fail(401, String::from("Unauthorized"), String::from("Invalid token"))),
    };

    match data {
        Ok(d) => {
            let new = TreatmentForm {
                ..d.into_inner()
            };
            let result = TreatmentForm::create(
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

#[put("/", data = "<data>")]
fn update(
    data: Result<Json<TreatmentForm>, JsonError>,
    conn: DbConn, 
    key: ApiKey
) -> Result<ApiResponse, ApiError> {
    let claim = match get_claim(key.key.as_str()) {
        Some(c) => c,
        None => return Err(fail(401, String::from("Unauthorized"), String::from("Invalid token"))),
    };

    match data {
        Ok(d) => {
            let new = TreatmentForm {
                ..d.into_inner()
            };
            let result = TreatmentForm::update(
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

#[delete("/", data = "<data>")]
fn delete(
    data: Result<Json<TreatmentDeleteForm>, JsonError>,
    conn: DbConn, 
    key: ApiKey
) -> Result<ApiResponse, ApiError> {
    let claim = match get_claim(key.key.as_str()) {
        Some(c) => c,
        None => return Err(fail(401, String::from("Unauthorized"), String::from("Invalid token"))),
    };

    match data {
        Ok(d) => {
            let new = TreatmentDeleteForm {
                ..d.into_inner()
            };
            let result = TreatmentDeleteForm::delete(
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
    routes![read, create, update, delete]
}