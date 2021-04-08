use chrono::{NaiveDate, NaiveDateTime, Utc};
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::Error;

use crate::schema::user_accounts;


#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[table_name = "user_accounts"]
pub struct UserAccount {
    pub account_id: Option<i32>,
    pub email: String,
    pub password: String,
    pub create_date: Option<NaiveDate>,
    pub last_login: Option<NaiveDateTime>,
}

impl UserAccount {
    pub fn read(conn: &MysqlConnection) -> Result<Vec<UserAccount>, Error> {
        user_accounts::table.load::<UserAccount>(conn)
    }

    pub fn read_by_id(id: i32, conn: &MysqlConnection) -> Result<UserAccount, Error> {
        user_accounts::table.find(id).first(conn)
    }

    pub fn read_by_email(user_email: String, conn: &MysqlConnection) -> Result<UserAccount, Error> {
        user_accounts::table.filter(user_accounts::email.eq(user_email)).first(conn)
    }

    pub fn create(user: UserAccount, conn: &MysqlConnection) -> Result<UserAccount, Error> {
        let new_user = UserAccount {
            create_date: Some(Utc::now().naive_utc().date()),
            last_login: Some(Utc::now().naive_utc()),
            ..user
        };

        let ops = diesel::insert_into(user_accounts::table)
            .values(&new_user)
            .execute(conn);

        match ops {
            Ok(_) => user_accounts::table.order(user_accounts::account_id.desc()).first(conn),
            Err(e) => Err(e),
        }
    }

    pub fn update(id: i32, user: UserAccount, conn: &MysqlConnection) -> Result<UserAccount, Error> {
        let new_user = UserAccount {
            ..user
        };

        let ops = diesel::update(user_accounts::table.find(id))
            .set(&new_user)
            .execute(conn);

        match ops {
            Ok(_) => user_accounts::table.find(id).first(conn),
            Err(e) => Err(e),
        }
    }

    pub fn delete(id: i32, conn: &MysqlConnection) -> bool {
        diesel::delete(user_accounts::table.find(id)).execute(conn).is_ok()
    }
}