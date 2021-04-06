use crate::models::database::user_accounts::UserAccount;
use crate::utils::response::*;
// use crate::*;
use rocket_contrib::json::{Json, JsonError};
use crate::DbConn;

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

// TODO: update this so it takes a jwt::ApiKey and updates BOTH UserAccounts AND UserInfo
#[put("/<id>", data = "<user>")]
fn update(
    id: i32,
    user: Result<Json<UserAccount>, JsonError>,
    conn: DbConn,
) -> Result<ApiResponse, ApiError> {
    match user {
        Ok(u) => {
            let update = UserAccount {
                account_id: Some(id),
                ..u.into_inner()
            };
            let result = UserAccount::update(id, update, &conn);
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