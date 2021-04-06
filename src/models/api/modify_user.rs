use diesel::mysql::MysqlConnection;
use chrono::NaiveDate;
use diesel::result::Error;

use crate::models::database::{user_accounts::UserAccount, user_info::UserInfo};

#[derive(Serialize, Deserialize)]
pub struct ModifyUserData {
    pub email: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    pub gender: Option<String>,
    pub birthday: Option<NaiveDate>,
}

impl ModifyUserData {
    pub fn update(data: ModifyUserData, email: String, conn: &MysqlConnection) -> Result<(), Error> {
        let mut user_account = match UserAccount::read_by_email(email, conn) {
            Ok(a) => a,
            Err(e) => return Err(e)
        };

        let mut user_info = match UserInfo::read_by_account_id(user_account.account_id.unwrap(), conn) {
            Ok(i) => i,
            Err(e) => return Err(e)
        };

        if let Some(e) = data.email {
            user_account.email = e;
        }

        if let Some(p) = data.password {
            user_account.password = p;
        }

        user_info.name = data.name;

        user_info.gender = data.gender;
        
        user_info.birthday = data.birthday;

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