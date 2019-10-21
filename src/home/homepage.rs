use rocket::http::Cookies;
use rocket::request::Form;
use std::collections::HashMap;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;

use crate::db::post;
use crate::db::comment;
use crate::db::category;
use crate::session::session::Session;
use crate::form::comment::FormInput;

#[derive(Debug, Serialize)]
struct IndexTemplate<'a, 'b> {
    cookie: &'a HashMap<&'a str, &'b str>,
    posts: HashMap<&'a str, post::Posts>,
    categories: Vec<category::Category>,
}

#[derive(Debug, Serialize)]
struct PostTemplate {
    post: post::Post,
    comments: Vec<comment::Comment>,
    categories: Vec<category::Category>,
}

#[get("/")]
pub fn index(_session: Session, cookies: Cookies) -> Template {
    println!("Sessiooooooooooooon : {:?}", _session);
    home(cookies, 1, "".to_string())
}

#[get("/page/<page>")]
pub fn page(_session: Session, cookies: Cookies, page: i32) -> Template {
    home(cookies, page, "".to_string())
}

#[get("/category/<category>")]
pub fn category(_session: Session, cookies: Cookies, category: String) -> Template {
    println!("Sessiooooooooooooon : {:?}", _session);
    home(cookies, 1, category)
}

fn home(mut _cookies: Cookies, page: i32, filter: String) -> Template
{
    let mut context = HashMap::<&str, &str>::new();
    let username = _cookies.get_private("username");

    if let Some(ref username) = username {
        context.insert("username", username.value());
    }

    let posts: post::Posts = post::posts(page, filter);
    let mut posts_data = HashMap::<&str, post::Posts>::new();
    posts_data.insert("posts", posts);


    let data = IndexTemplate {
        cookie: &context,
        posts: posts_data,
        categories: category::all(),
    };
    // println!("{:#?}", data);
    Template::render("homepage/index", data)
}

#[get("/note/<id>")]
pub fn post(id: i32) -> Template {
    let post: post::Post = post::get_post(id);

    /*println!("{:?}", post);*/

    Template::render("homepage/post", PostTemplate {
        post: post,
        comments: comment::Comment::get_by_post(id),
        categories: category::all(),
    })
}

#[post("/note/<id>", data = "<comment>")]
pub fn comment(comment: Form<FormInput>, id: i32) -> Result<Redirect, String> {
    println!("{:?}", &comment);
    comment::create(comment, id);

    /*let redirect = "/note/".to_string() + &id.to_string();*/
    let redirect = format!("/note/{}", id);

    /*println!("{:?}", &redirect);*/
    Ok(Redirect::to(redirect))
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
