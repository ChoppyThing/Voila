use rocket::Outcome;
use rocket::request::{self, FromRequest, Request};

#[derive(Debug)]
pub struct Session {
    id: i8,
    username: String,
    role: String
}

#[derive(Debug)]
pub enum Error {
    Error
}

// Define if user is logged in
impl<'a, 'r> FromRequest<'a, 'r> for Session {
    type Error = Error;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let mut user = Session {
            id: 0,
            username: "default".to_string(),
            role: "default".to_string(),
        };

        request.cookies().get_private("username").map(|cookie| user.username = cookie.value().to_string());

        Outcome::Success(user)
    }
}
