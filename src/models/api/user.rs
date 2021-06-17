use diesel::mysql::MysqlConnection;
use diesel::result::Error;

use crate::models::database::{user_account::UserAccount, user_info::UserInfo};

#[derive(Serialize, Deserialize)]
pub struct UserData {
    pub email: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    pub gender: Option<String>,
    pub age: Option<i32>,
}

impl UserData {
    pub fn read(email: String, conn: &MysqlConnection) -> Result<UserData, Error> {
        let user_account = UserAccount::read_by_email(email, conn)?;
        let user_info = UserInfo::read_by_account_id(user_account.account_id.unwrap(), conn)?;

        Ok(UserData {
            email: Some(user_account.email),
            password: Some(user_account.password),
            name: user_info.name,
            gender: user_info.gender,
            age: user_info.age,
        })
    }

    pub fn update(data: UserData, email: String, conn: &MysqlConnection) -> Result<(), Error> {
        let mut user_account = UserAccount::read_by_email(email, conn)?;
        let mut user_info = UserInfo::read_by_account_id(user_account.account_id.unwrap(), conn)?;

        if let Some(e) = data.email {
            user_account.email = e;
        }

        if let Some(p) = data.password {
            user_account.password = p;
        }

        user_info.name = data.name;

        user_info.gender = data.gender;

        user_info.age = data.age;

        let ret = UserAccount::update(
            user_account.account_id.unwrap(),
            user_account,
            conn
        );

        if let Err(e) = ret {
            return Err(e);
        }

        let ret = UserInfo::update(
            user_info.user_id.unwrap(),
            user_info,
            conn
        );

        if let Err(e) = ret {
            return Err(e);
        }

        Ok(())
    }
}