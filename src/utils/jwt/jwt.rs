use chrono::{Duration, Utc};

extern crate jsonwebtoken;

use self::jsonwebtoken::{encode, Header, EncodingKey};
use crate::utils::jwt::claim::{get_claim, Claim};
use crate::models::database::user_account::UserAccount;
use std::env;

pub static JWT_KEY: &str = "very_secret";

pub type Token = String;

pub fn generate_jwt(user: &UserAccount) -> Option<Token> {
    let secret = match env::var("KEY") {
        Ok(val) => val,
        Err(_) => {
            println!("Setting to default secret for jwt");
            JWT_KEY.to_string()
        }
    };
    // println!("secret is {}", secret);

    let claim = Claim {
        exp: (Utc::now() + Duration::weeks(43)),
        iat: Utc::now(),
        iss: "Medi".to_string(),
        usr:  user.email.clone(),
    };

    match encode(&Header::default(), &claim, &EncodingKey::from_secret(secret.as_ref())) {
        Ok(jwt) => Some(jwt),
        Err(_) => None,
    }
}

pub fn is_valid(jwt: &str) -> bool {
    get_claim(jwt).is_some()
}