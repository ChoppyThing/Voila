use std::collections::HashMap;
use rocket_contrib::templates::Template;
use rocket::outcome::IntoOutcome;
use rocket::http::{Cookie, Cookies};
use rocket::response::{Redirect, Flash};
use rocket::request::{self, Form, FlashMessage, FromRequest, Request};

use crate::db::user::User as UserEntity;

// mod home;

#[derive(FromForm)]
pub struct Login {
    pub username: String,
    password: String
}

#[derive(Debug)]
pub struct User(pub String);

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = std::convert::Infallible;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<User, Self::Error> {
        request.cookies()
            .get_private("username")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(|username| User(username))
            .or_forward(())
    }
}

#[get("/login")]
pub fn login_page(flash: Option<FlashMessage<'_, '_>>) -> Template {
    let mut context = HashMap::new();
    if let Some(ref msg) = flash {
        context.insert("flash", msg.msg());
        if msg.name() == "error" {
            context.insert("flash_type", "Error");
        }
    }

    Template::render("admin/login", &context)
}

#[post("/login", data = "<login>")]
pub fn login(mut cookies: Cookies<'_>, login: Form<Login>) -> Result<Redirect, Flash<Redirect>> {

    let user = UserEntity::login(&login.username, &login.password);
    println!("Heyyah{:?}", user);

    match &user {
        Some(_user_data) => {
            let user = user.unwrap();
            // println!("USEEER {:?}", &user);
            cookies.add_private(Cookie::new("id", user.id.to_string()));
            cookies.add_private(Cookie::new("username", user.username));
            // println!("Cookiiiies {:?}", cookies);
            Ok(Redirect::to(uri!(crate::home::homepage::index)))
        },
        None => Err(Flash::error(Redirect::to(uri!(login_page)), "Invalid username/password."))
    }

    /*if login.username == "choppy" && login.password == "test" {
        cookies.add_private(Cookie::new("username", login.username.to_string()));
        Ok(Redirect::to(uri!(crate::home::homepage::index)))
    } else {
        
    }*/
}

#[get("/logout")]
pub fn logout(mut cookies: Cookies<'_>) -> Flash<Redirect> {
    cookies.remove_private(Cookie::named("id"));
    cookies.remove_private(Cookie::named("username"));
    Flash::success(Redirect::to(uri!(login_page)), "Successfully logged out.")
}
