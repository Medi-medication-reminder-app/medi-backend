use rocket_contrib::json::{Json, JsonError};

use crate::models::api::{signup::SignupForm, login::LoginForm};
use crate::DbConn;
use crate::utils::response::*;

#[post("/signup", data = "<form>")]
fn signup(
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
                Ok(r) => {
                    let jwt = LoginForm::login(r, &conn);
                    match jwt {
                        Ok(Some(t)) => Ok(success(json!(t))),
                        Ok(None) => Err(fail(403, String::from("Forbidden"), String::from("Invalid password"))),
                        Err(e) => Err(db_error(e)),
                    }
                },
                Err(e) => Err(db_error(e)),
            }
        }
        Err(e) => Err(json_error(e)),
    }
}

#[post("/login", data = "<form>")]
fn login(
    form: Result<Json<LoginForm>, JsonError>,
    conn: DbConn,
) -> Result<ApiResponse, ApiError> {
    match form {
        Ok(f) => {
            let login_info = LoginForm {
                ..f.into_inner()
            };
            let result = LoginForm::login(login_info, &conn);
            match result {
                Ok(Some(r)) => Ok(success(json!(r))),
                Ok(None) => Err(fail(403, String::from("Forbidden"), String::from("Invalid password"))),
                Err(e) => Err(db_error(e)),
            }
        }
        Err(e) => Err(json_error(e)),
    }
}

// -- routes
pub fn routes() -> Vec<rocket::Route> {
    routes![signup, login]
}