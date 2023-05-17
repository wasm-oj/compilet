use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use rocket::request::{self, FromRequest, Request};
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

use crate::config::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Claims {
    exp: usize,
}

pub fn is_valid_token(token: &str) -> bool {
    let secret = app_secret();

    let validation = Validation::new(Algorithm::HS256);

    let token = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    );

    token.is_ok()
}

pub struct Token(String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        if keys.len() != 1 {
            return request::Outcome::Failure((rocket::http::Status::Unauthorized, ()));
        }

        let key = keys[0];
        let key = key.to_string();
        let key = key.replace("Bearer ", "");

        let valid = is_valid_token(&key);

        if valid {
            request::Outcome::Success(Token(key))
        } else {
            request::Outcome::Failure((rocket::http::Status::Unauthorized, ()))
        }
    }
}

/// Check if the given token (in auth header) is valid
#[get("/validate")]
pub fn validate(_token: Token) -> Json<bool> {
    Json(true)
}
