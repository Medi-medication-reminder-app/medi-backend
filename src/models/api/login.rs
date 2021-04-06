use diesel::mysql::MysqlConnection;
use diesel::result::Error;

use crate::models::database::user_accounts::UserAccount;
use crate::utils::jwt::jwt::{Token, generate_jwt};

#[derive(Serialize, Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

impl LoginForm {
    pub fn login(form: LoginForm, conn: &MysqlConnection) -> Result<Option<Token>, Error> {
        match UserAccount::read_by_email(form.email, conn) {
            Ok(u) if u.password == form.password => Ok(generate_jwt(&u)),
            Ok(_) => Ok(None),
            Err(e) => Err(e),
        }
    }
}