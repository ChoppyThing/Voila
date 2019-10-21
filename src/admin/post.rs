use std::collections::HashMap;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;
use rocket::request::Form;

use crate::form::post::FormInput;
use crate::db::post;
use crate::db::category;
use crate::session::admin::Connected;

#[derive(Debug, Serialize)]
struct IndexTemplate<'a> {
    posts: HashMap<&'a str, post::Posts>
}

#[derive(Debug, Serialize)]
struct PostEditTemplate {
    post: post::Post,
    categories: Vec<category::Category>,
}

#[get("/admin/post")]
pub fn index(_connected: Connected) -> Template {
    list(1)
}

#[get("/admin/post/page/<page>")]
pub fn page(_connected: Connected, page: i32) -> Template {
    list(page)
}

fn list(page: i32) -> Template
{
    let posts: post::Posts = post::admin_posts(page, "".to_string());
    let mut posts_data = HashMap::<&str, post::Posts>::new();
    posts_data.insert("posts", posts);


    let data = IndexTemplate {
        posts: posts_data
    };
    println!("{:#?}", data);
    Template::render("admin/list", data)
}


#[get("/admin/post/new")]
pub fn new(_connected: Connected) -> Template {
    let categories: Vec<category::Category> = category::all();
    let mut data = HashMap::<&str, Vec<category::Category>>::new();
    data.insert("categories", categories);

    Template::render("admin/new", &data)
}

#[post("/admin/post/new", data = "<post>")]
pub fn add(_connected: Connected, post: Form<FormInput>) -> Result<Redirect, String> {
    println!("{:?}", &post);
    post::create(post);

    Ok(Redirect::to("/admin/post"))
}

#[get("/admin/post/<id>")]
pub fn edit(_connected: Connected, id: i32) -> Template {
    edit_update(id)
}

#[post("/admin/post/<id>", data = "<post>")]
pub fn update(_connected: Connected, post: Form<FormInput>, id: i32) -> Template {
    post::update(post, id);

    edit_update(id)
}

fn edit_update(id: i32) -> Template {
    let post_data = post::get_post(id);
    let categories: Vec<category::Category> = category::all();

    let data = PostEditTemplate {
        post: post_data,
        categories: categories
    };

println!("{:?}", &data);
    Template::render("admin/edit", &data)
}
