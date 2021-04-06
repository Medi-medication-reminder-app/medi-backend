use rocket_contrib::json::{Json, JsonError};

use crate::models::api::{signup::SignupForm, login::LoginForm};
use crate::DbConn;
use crate::utils::response::*;

fn _login(data: LoginForm, conn: &DbConn) -> Result<ApiResponse, ApiError> {
    let jwt = LoginForm::login(data, &conn);
    match jwt {
        Ok(Some(t)) => Ok(success(json!(t))),
        Ok(None) => Err(fail(403, String::from("Forbidden"), String::from("Invalid password"))),
        Err(e) => Err(db_error(e)),
    }
}

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
                Ok(r) => _login(r, &conn),
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
            _login(login_info, &conn)
        }
        Err(e) => Err(json_error(e)),
    }
}

// -- routes
pub fn routes() -> Vec<rocket::Route> {
    routes![signup, login]
}