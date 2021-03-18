use crate::models::database::user_accounts::UserAccount;
use crate::utils::response::*;
// use crate::*;
use rocket_contrib::json::{Json, JsonError};
use crate::DbConn;

#[get("/")]
fn read(conn: DbConn) -> Result<ApiResponse, ApiError> {
    let result = UserAccount::read(&conn);
    match result {
        Ok(r) => Ok(success(json!(r))),
        Err(e) => Err(db_error(e)),
    }
}

#[get("/<id>")]
fn read_by_id(id: i32, conn: DbConn) -> Result<ApiResponse, ApiError> {
    let result = UserAccount::read_by_id(id, &conn);
    match result {
        Ok(r) => Ok(success(json!(r))),
        Err(e) => Err(db_error(e)),
    }
}

#[post("/", data = "<user>")]
fn create(
    user: Result<Json<UserAccount>, JsonError>,
    conn: DbConn,
) -> Result<ApiResponse, ApiError> {
    match user {
        Ok(u) => {
            let insert = UserAccount {
                account_id: None,
                ..u.into_inner()
            };
            let result = UserAccount::create(insert, &conn);
            match result {
                Ok(r) => Ok(success(json!(r))),
                Err(e) => Err(db_error(e)),
            }
        }
        Err(e) => Err(json_error(e)),
    }
}

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

#[delete("/<id>")]
fn delete(id: i32, conn: DbConn) -> ApiResponse {
    let result = UserAccount::delete(id, &conn);
    success(json!(result))
}

// -- routes
pub fn routes() -> Vec<rocket::Route> {
    routes![read, read_by_id, create, update, delete]
}