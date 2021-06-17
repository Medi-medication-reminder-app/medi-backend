use rocket_contrib::json::{Json, JsonError};
use crate::models::api::user::UserData;
use crate::utils::response::*;
use crate::DbConn;
use crate::utils::jwt::{apikey::ApiKey, claim::get_claim};

#[get("/")]
fn read_by_id(conn: DbConn, key: ApiKey) -> Result<ApiResponse, ApiError> {
    let claim = match get_claim(key.key.as_str()) {
        Some(c) => c,
        None => return Err(fail(401, String::from("Unauthorized"), String::from("Invalid token"))),
    };

    let result = UserData::read(claim.usr, &conn);
    match result {
        Ok(r) => Ok(success(json!(r))),
        Err(e) => Err(db_error(e)),
    }
}

#[put("/", data = "<data>")]
fn update(
    data: Result<Json<UserData>, JsonError>,
    conn: DbConn,
    key: ApiKey,
) -> Result<ApiResponse, ApiError> {
    let claim = match get_claim(key.key.as_str()) {
        Some(c) => c,
        None => return Err(fail(401, String::from("Unauthorized"), String::from("Invalid token"))),
    };

    match data {
        Ok(d) => {
            let modify = UserData {
                ..d.into_inner()
            };
            let result = UserData::update(modify, claim.usr, &conn);
            match result {
                Ok(r) => Ok(success(json!(r))),
                Err(e) => Err(db_error(e)),
            }
        }
        Err(e) => Err(json_error(e)),
    }
}

pub fn routes() -> Vec<rocket::Route> {
    routes![read_by_id, update]
}