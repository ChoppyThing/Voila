use rocket::request::{Request, FromRequest};
use rocket::http::Status;
use rocket::Outcome;
use rocket::request;

pub struct Connected(String);

#[derive(Debug)]
pub enum ConnectionError {
    Disconnected,
}

// Secured space
impl<'a, 'r> FromRequest<'a, 'r> for Connected {
    type Error = ConnectionError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let username = request.cookies().get_private("username");

        if username.is_none() {
            println!("C'est None : {:?}", username);
            return Outcome::Failure((Status::BadRequest, ConnectionError::Disconnected));
        } else if username.is_some() {
            println!("C'est some : {:#?}", username);
            return Outcome::Success(Connected(username.unwrap().to_string()));
        }

        Outcome::Failure((Status::BadRequest, ConnectionError::Disconnected))
    }
}
