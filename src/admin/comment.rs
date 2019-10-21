use rocket_contrib::templates::Template;
use rocket::response::Redirect;

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
    // println!("{:#?}", data);

    Template::render("admin/comment/list", data)
}

#[get("/admin/comment/remove/<comment>")]
pub fn remove(_connected: Connected, comment: i32) -> Result<Redirect, String> {
	comment::remove(comment);
    Ok(Redirect::to("/admin/comment"))
}
