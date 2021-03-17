use crate::models::user_accounts::UserAccounts;
use crate::utils::response::*;
// use crate::*;
use rocket_contrib::json::{Json, JsonError};
use crate::DbConn;

#[get("/a")]
fn read(conn: DbConn) -> Result<ApiResponse, ApiError> {
    let result = UserAccounts::read(&conn);
    match result {
        Ok(r) => Ok(success(json!(r))),
        Err(e) => Err(db_error(e)),
    }
}

// #[get("/a")]
// pub fn list_page_views(conn: DbConn) -> Result<Json<Vec<UserAccounts>>, String> {
//     use crate::schema::user_accounts::dsl::*;
//     // use crate::schema::user_accounts;

//     user_accounts::table.load::<UserAccounts>(&conn.0).map_err(|err| -> String {
//         println!("Error querying page views: {:?}", err);
//         "Error querying page views from the database".into()
//     }).map(Json)
// }

// -- routes
pub fn routes() -> Vec<rocket::Route> {
    routes![read]
}