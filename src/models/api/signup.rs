use diesel::mysql::MysqlConnection;
use diesel::result::Error;

use crate::models::database::{user_account::UserAccount, user_info::UserInfo, caretaker::Caretaker};
use crate::models::api::login::LoginForm;

#[derive(Serialize, Deserialize)]
pub struct SignupForm {
    pub email: String,
    pub password: String,
    pub name: Option<String>,
    pub gender: Option<String>,
    pub age: Option<i32>,
    pub caretaker: Option<Caretaker>,
}

impl SignupForm {
    pub fn create(form: SignupForm, conn: &MysqlConnection) -> Result<LoginForm, Error> {
        // add user account to user_accounts table
        let user_account = UserAccount {
            account_id: None,
            email: form.email,
            password: form.password,
            create_date: None,
            last_login: None,
        };
        match UserAccount::create(user_account, conn) {
            Ok(a) => {
                //if successful, add user info into user_info table
                let user_info = UserInfo {
                    user_id: None,
                    account_id: a.account_id.unwrap(),  // because it matched in Ok, there must be an account_id
                    name: form.name,
                    gender: form.gender,
                    age: form.age,
                };
                match UserInfo::create(user_info, conn) {
                    Ok(i) => {
                        // aaaand if everything went alright, add caretaker (should it exist)
                        if let Some(c) = form.caretaker {
                            let caretaker = Caretaker {
                                caretaker_id: None,
                                user_id: i.user_id,
                                name: c.name,
                                phone_number: c.phone_number,
                            };
                            if let Err(e) = Caretaker::create(caretaker, conn) {
                                return Err(e);
                            }
                        }
                        // return login information to auto-login
                        let login = LoginForm {
                            email: a.email,
                            password: a.password,
                        };
                        // Good return case
                        Ok(login)
                    },
                    Err(e) => Err(e),
                }

            }
            Err(e) => Err(e),
        }

    }
}