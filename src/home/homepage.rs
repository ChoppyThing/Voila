use std::collections::HashMap;
use rocket_contrib::templates::Template;
use rocket::http::Cookies;

use crate::db::post;
use crate::session::session::Session;

#[derive(Debug, Serialize)]
struct IndexTemplate<'a, 'b> {
    cookie: &'a HashMap<&'a str, &'b str>,
    posts: HashMap<&'a str, post::Posts>
}

#[get("/")]
pub fn index(_session: Session, cookies: Cookies) -> Template {
    println!("Sessiooooooooooooon : {:?}", _session);
    home(cookies, 1)
}

#[get("/page/<page>")]
pub fn page(_session: Session, cookies: Cookies, page: i32) -> Template {
    home(cookies, page)
}

fn home(mut _cookies: Cookies, page: i32) -> Template
{
    let mut context = HashMap::<&str, &str>::new();
    let username = _cookies.get_private("username");

    if let Some(ref username) = username {
        context.insert("username", username.value());
    }

    let posts: post::Posts = post::posts(page);
    let mut posts_data = HashMap::<&str, post::Posts>::new();
    posts_data.insert("posts", posts);


    let data = IndexTemplate {
        cookie: &context,
        posts: posts_data
    };
    // println!("{:#?}", data);
    Template::render("homepage/index", data)
}

#[get("/test")]
pub fn index_test(user: crate::user::user::User) -> Template {
    // let context = HashMap::<&str, &str>::new();
    let mut context = HashMap::<&str, &str>::new();
    context.insert("username", &user.0);
    Template::render("index", &context)
}

#[get("/world?<name>")]
pub fn world(_cookies: Cookies, name: String) -> String {
    println!("{:?}", _cookies);
    format!("Hello, worldssss {}!", name)
}
