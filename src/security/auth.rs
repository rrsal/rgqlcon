use std::fmt;
use chrono::prelude::*;
use chrono::Duration;
use jsonwebtoken::{decode,encode,Algorithm,DecodingKey,EncodingKey,Header,Validation};
use serde::{Deserialize,Serialize};
use warp::{
    filters::header::headers_cloned,
    http::header::{HeaderMap, HeaderValue, AUTHORIZATION},
    reject, Filter, Rejection,
};

use crate::{error::Error};

const BEARER: &str = "Bearer ";
const JWT_SECRET: &[u8] = b"Basic Secret, not even secret";

type Result<T> = std::result::Result<T,Error>;
type WebResult<T> = std::result::Result<T, Rejection>;

// Creating an enum for role
// So that it can be used at place of strings
#[derive(Clone,PartialEq)]
pub enum Role{
    User,
    Admin
}

// This converts string input to enum role
impl Role{
    pub fn from_str(role: &str) -> Role {
        match role {
            "Admin" => Role::Admin,
            _ => Role::User
        }
    }
}


// This converts enum role to it's respective String
// So that it can be used to create a token with role
impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::User => write!(f, "User"),
            Role::Admin => write!(f, "Admin"),
        }
    }
}


// Type Claim represents the data we save as JWT and 
// Expects out of our JWT. 

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Claims{
    sub: String,            // Who
    role: String,          // Type of role of who
    iat: DateTime<Utc>,   // Initaited AT
    exp: usize,          // Expiration date/time
}


// This function normalizes the `iat` timestamps for Claims.
impl Claims {
    pub fn new(sub:String,role:String,iat: DateTime<Utc>,exp:usize) -> Self {
        let iat = iat.date().and_hms_milli(iat.hour(), iat.minute(), iat.second(), 0);
        Self {sub,role,iat,exp}
    }
}

// Serialization & Deserialization of `iat`
// To & From : DateTime<Utc> = Unix TimeStamp
mod jwt_numeric_date{
    use chrono::{DateTime,TimeZone,Utc};
    use serde::{self,Deserialize,Deserializer,Serializer};

    // Serialization
    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let timestamp = date.timestamp();
        serializer.serialize_i64(timestamp)
    }

    // Deserialization
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Utc.timestamp_opt(i64::deserialize(deserializer)?, 0)
            .single() // If there are multiple or no valid DateTimes from timestamp, return None
            .ok_or_else(|| serde::de::Error::custom("invalid Unix timestamp value"))
    }
}



pub fn create_jwt(uid:&str, role: &Role) -> Result<String> {
    let expiration = Utc::now()
    .checked_add_signed(Duration::minutes(120))
    .expect("Valid TimeStamp")
    .timestamp();

    let claims = Claims{
        sub: uid.to_owned(),
        role: role.to_string(),
        iat: Utc.timestamp(0,0),
        exp: expiration as usize,
    };

    let header = Header::new(Algorithm::HS512);

    encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET))
    .map_err(|_| Error::JWTTokenCreationError)
}



fn jwt_from_header(headers: &HeaderMap<HeaderValue>) -> Result<String> {
    let header = match headers.get(AUTHORIZATION) {
        Some(v) => v,
        None => return Err(Error::NoAuthHeaderError),
    };
    let auth_header = match std::str::from_utf8(header.as_bytes()) {
        Ok(v) => v,
        Err(_) => return Err(Error::NoAuthHeaderError),
    };
    if !auth_header.starts_with(BEARER) {
        return Err(Error::InvalidAuthHeaderError);
    }
    Ok(auth_header.trim_start_matches(BEARER).to_owned())
}



async fn authorize((role, headers): (Role, HeaderMap<HeaderValue>)) -> WebResult<String> {
    match jwt_from_header(&headers) {
        Ok(jwt) => {
            let decoded = decode::<Claims>(
                &jwt,
                &DecodingKey::from_secret(JWT_SECRET),
                &Validation::new(Algorithm::HS512),
            )
            .map_err(|_| reject::custom(Error::JWTTokenError))?;

            if role == Role::Admin && Role::from_str(&decoded.claims.role) != Role::Admin {
                return Err(reject::custom(Error::NoPermissionError));
            }

            Ok(decoded.claims.sub)
        }
        Err(e) => return Err(reject::custom(e)),
    }
}

async fn authorize_token(claims:Claims,role:Role) ->  WebResult<String> {
    let header = Header::new(Algorithm::HS512);
    match encode(&header, &claims,&EncodingKey::from_secret(JWT_SECRET)){
        Ok(token) =>{
            let decoded = decode::<Claims>(
                &token,
                &DecodingKey::from_secret(JWT_SECRET),
                &Validation::new(Algorithm::HS512),
            )
            .map_err(|_| reject::custom(Error::JWTTokenError))?;

            if role == Role::Admin && Role::from_str(&decoded.claims.role) != Role::Admin {
                return Err(reject::custom(Error::NoPermissionError));
            }

            Ok(decoded.claims.sub)
        },
        Err(_) => Err(reject::custom(Error::NoPermissionError)),
    }
}


