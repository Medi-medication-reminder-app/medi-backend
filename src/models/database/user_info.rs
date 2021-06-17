
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::Error;

use crate::schema::user_info;

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[table_name = "user_info"]
pub struct UserInfo {
    pub user_id: Option<i32>,
    pub account_id: i32,
    pub name: Option<String>,
    pub gender: Option<String>,
    pub age: Option<i32>,
}

impl UserInfo {
    pub fn read(conn: &MysqlConnection) -> Result<Vec<UserInfo>, Error> {
        user_info::table.load::<UserInfo>(conn)
    }

    pub fn read_by_id(id: i32, conn: &MysqlConnection) -> Result<UserInfo, Error> {
        user_info::table.find(id).first(conn)
    }

    pub fn read_by_account_id(id: i32, conn: &MysqlConnection) -> Result<UserInfo, Error> {
        user_info::table.filter(user_info::account_id.eq(id)).first(conn)
    }

    pub fn create(user: UserInfo, conn: &MysqlConnection) -> Result<UserInfo, Error> {
        let new_user = UserInfo {
            ..user
        };

        let ops = diesel::insert_into(user_info::table)
            .values(&new_user)
            .execute(conn);

        match ops {
            Ok(_) => user_info::table.order(user_info::user_id.desc()).first(conn),
            Err(e) => Err(e),
        }
    }

    pub fn update(id: i32, user: UserInfo, conn: &MysqlConnection) -> Result<UserInfo, Error> {
        let new_user = UserInfo {
            ..user
        };

        let ops = diesel::update(user_info::table.find(id))
            .set(&new_user)
            .execute(conn);

        match ops {
            Ok(_) => user_info::table.find(id).first(conn),
            Err(e) => Err(e),
        }
    }

    pub fn delete(id: i32, conn: &MysqlConnection) -> bool {
        diesel::delete(user_info::table.find(id)).execute(conn).is_ok()
    }
}