use chrono::{DateTime, Utc};
use jsonwebtoken::{decode, Validation, DecodingKey};
use std::env;

use crate::utils::jwt::jwt::JWT_KEY;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Claim {
    #[serde(with = "jwt_numeric_date")]
    pub(crate) exp: DateTime<Utc>,
    #[serde(with = "jwt_numeric_date")]
    pub(crate) iat: DateTime<Utc>,
    pub(crate) iss: String,
    pub usr: String,
}

pub fn get_claim(jwt: &str) -> Option<Claim> {
    let jwt: &str = match jwt.split_ascii_whitespace().last() {
        Some(token) => token,
        None => return None,
    };
    println!("JWT is: {:?}", jwt);

    let secret = match env::var("KEY") {
        Ok(val) => val,
        Err(_) => {
            println!("Setting to default secret for jwt");
            JWT_KEY.to_string()
        }
    };

    // let decoded = decode::<Claim>(&jwt, secret.as_ref(), &Validation::default());
    let decoded = decode::<Claim>(&jwt, &DecodingKey::from_secret(secret.as_ref()), &Validation::default());

    let token = match decoded {
        Ok(token) => token,
        Err(e) => {
            println!("Incorrect jwt {:?}", e);
            return None;
        }
    };

    println!("Claim is: {:?}", token.claims);

    Some(token.claims)
}

mod jwt_numeric_date {
    //! Custom serialization of DateTime<Utc> to conform with the JWT spec (RFC 7519 section 2, "Numeric Date")
    use chrono::{DateTime, TimeZone, Utc};
    #[allow(unused_imports)]
    use serde::de::Error;
    use serde::{self, Deserialize, Deserializer, Serializer};

    /// Serializes a DateTime<Utc> to a Unix timestamp (milliseconds since 1970/1/1T00:00:00T)
    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let timestamp = date.timestamp();
        serializer.serialize_i64(timestamp)
    }

    /// Attempts to deserialize an i64 and use as a Unix timestamp
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
        where
            D: Deserializer<'de>,
    {
        Utc.timestamp_opt(i64::deserialize(deserializer)?, 0)
            .single() // If there are multiple or no valid DateTimes from timestamp, return None
            .ok_or_else(|| serde::de::Error::custom("invalid Unix timestamp value"))
    }

//     #[cfg(test)]
//     mod tests {
//         const EXPECTED_TOKEN: &str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJDdXN0b20gRGF0ZVRpbWUgc2VyL2RlIiwiaWF0IjowLCJleHAiOjMyNTAzNjgwMDAwfQ.RTgha0S53MjPC2pMA4e2oMzaBxSY3DMjiYR2qFfV55A";

//         use jsonwebtoken::{decode, encode, Header, Validation};
//         use super::super::{Claims, SECRET};
//         use chrono::{TimeZone, Utc};
//         use jsonwebtoken::{decode, encode, Header, Validation};

//         #[test]
//         fn round_trip() {
//             let sub = "Custom DateTime ser/de".to_string();
//             let iat = Utc.timestamp(0, 0);
//             let exp = Utc.timestamp(32503680000, 0);

//             let claims = Claims {
//                 sub: sub.clone(),
//                 iat,
//                 exp,
//             };

//             let token = encode(
//                 &Header::default(),
//                 &claims,
//                 &EncodingKey::from_secret(SECRET.as_ref()),
//             )
//                 .expect("Failed to encode claims");

//             assert_eq!(&token, EXPECTED_TOKEN);

//             let decoded = decode::<Claims>(
//                 &token,
//                 &DecodingKey::from_secret(SECRET.as_ref()),
//                 &Validation::default(),
//             )
//                 .expect("Failed to decode token");

//             assert_eq!(decoded.claims, claims);
//         }

//         #[test]
//         fn should_fail_on_invalid_timestamp() {
//             // A token with the expiry of i64::MAX + 1
//             let overflow_token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJDdXN0b20gRGF0ZVRpbWUgc2VyL2RlIiwiaWF0IjowLCJleHAiOjkyMjMzNzIwMzY4NTQ3NzYwMDB9.G2PKreA27U8_xOwuIeCYXacFYeR46f9FyENIZfCrvEc";

//             let decode_result =
//                 decode::<Claims>(&overflow_token, SECRET.as_ref(), &Validation::default());

//             assert!(decode_result.is_err());
//         }
//     }
}