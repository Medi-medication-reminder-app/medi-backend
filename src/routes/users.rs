use rocket_contrib::json::{Json, JsonError};

use crate::models::database::user_account::UserAccount;
use crate::models::api::modify_user::ModifyUserData;
use crate::utils::response::*;
// use crate::*;
use crate::DbConn;
use crate::utils::jwt::{apikey::ApiKey, claim::get_claim};

// TODO: Delete this for release
#[get("/")]
fn read(conn: DbConn) -> Result<ApiResponse, ApiError> {
    let result = UserAccount::read(&conn);
    match result {
        Ok(r) => Ok(success(json!(r))),
        Err(e) => Err(db_error(e)),
    }
}

// TODO: Delete this for release
#[get("/<id>")]
fn read_by_id(id: i32, conn: DbConn) -> Result<ApiResponse, ApiError> {
    let result = UserAccount::read_by_id(id, &conn);
    match result {
        Ok(r) => Ok(success(json!(r))),
        Err(e) => Err(db_error(e)),
    }
}

#[put("/modify", data = "<data>")]
fn update(
    data: Result<Json<ModifyUserData>, JsonError>,
    conn: DbConn,
    key: ApiKey,
) -> Result<ApiResponse, ApiError> {
    let claim = match get_claim(key.key.as_str()) {
        Some(c) => c,
        None => return Err(fail(401, String::from("Unauthorized"), String::from("Invalid token"))),
    };
    
    match data {
        Ok(d) => {
            let modify = ModifyUserData {
                ..d.into_inner()
            };
            let result = ModifyUserData::update(modify, claim.usr, &conn);
            match result {
                Ok(r) => Ok(success(json!(r))),
                Err(e) => Err(db_error(e)),
            }
        }
        Err(e) => Err(json_error(e)),
    }
}

// -- routes
pub fn routes() -> Vec<rocket::Route> {
    routes![read, read_by_id, update]
}