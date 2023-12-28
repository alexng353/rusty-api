use crate::structs::User;
use anyhow::{anyhow, Error};
use rocket::{
    data::Outcome,
    http::Status,
    request::{self, FromRequest, Request},
};

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, ()> {
        let authorization = match req.headers().get_one("Authorization") {
            Some(authorization) => authorization,
            None => return Outcome::Error((Status::Unauthorized, ())),
        };
    }
}
