use rocket_contrib::json::{Json, JsonError};

use crate::models::api::signup::SignupForm;
use crate::DbConn;
use crate::utils::response::*;

#[post("/signup", data = "<form>")]
fn create(
    form: Result<Json<SignupForm>, JsonError>,
    conn: DbConn,
) -> Result<ApiResponse, ApiError> {
    match form {
        Ok(f) => {
            let insert = SignupForm {
                ..f.into_inner()
            };
            let result = SignupForm::create(insert, &conn);
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
    routes![create]
}