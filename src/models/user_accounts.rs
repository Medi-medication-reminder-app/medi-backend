use chrono::{NaiveDate, NaiveDateTime};
use diesel;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::Error;

use crate::schema::user_accounts;

#[derive(Serialize, Deserialize, Queryable)]
pub struct UserAccounts {
    pub account_id: i32,
    pub email: String,
    pub password: String,
    pub create_date: NaiveDate,
    pub last_login: NaiveDateTime,
}

#[derive(Deserialize, Insertable)]
#[table_name = "user_accounts"]
pub struct InsertableUserAccounts {
    pub email: String,
    pub password: String,
    pub create_date: NaiveDate,
    pub last_login: NaiveDateTime,
}

impl UserAccounts {
    pub fn read(conn: &MysqlConnection) -> Result<Vec<UserAccounts>, Error> {
        user_accounts::table.load::<UserAccounts>(conn)
    }
}