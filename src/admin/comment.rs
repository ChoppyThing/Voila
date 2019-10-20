use rocket_contrib::templates::Template;

use crate::db::comment;
use crate::session::admin::Connected;

#[derive(Debug, Serialize)]
struct ListTemplate {
    comments: Vec<comment::CommentPost>,
}


#[get("/admin/comment")]
pub fn index(_connected: Connected) -> Template {
    list(1)
}

#[get("/admin/comment/page/<page>")]
pub fn page(_connected: Connected, page: i32) -> Template {
    list(page)
}

fn list(page: i32) -> Template
{
    let comments: Vec<comment::CommentPost> = comment::Comment::get(page, 30);
    let data = ListTemplate {
        comments: comments
    };
    println!("{:#?}", data);

    Template::render("admin/comment/list", data)
}