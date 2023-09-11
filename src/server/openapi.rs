use rocket::request::{self, FromRequest, Request};

pub const OPENAPI_DOCUMENT: &str = std::include_str!("../../openapi.yml");

pub struct SelfRoot(pub String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for SelfRoot {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("Origin").collect();
        if keys.len() == 1 {
            return request::Outcome::Success(SelfRoot(keys[0].to_string()));
        }

        let keys: Vec<_> = request.headers().get("Host").collect();
        if keys.len() == 1 {
            return request::Outcome::Success(SelfRoot(format!("http://{}", keys[0])));
        }

        request::Outcome::Failure((rocket::http::Status::Unauthorized, ()))
    }
}
