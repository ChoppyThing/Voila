use std::collections::HashMap;
use rocket_contrib::templates::Template;
use rocket::request::Form;
use rocket::response::Redirect;

use crate::db::category;
use crate::form::category::FormInput;
use crate::session::admin::Connected;

#[derive(Debug, Serialize)]
struct ContextTemplate<'a> {
    categories: HashMap<&'a str, Vec<category::Category>>
}


#[get("/admin/category")]
pub fn index(_connected: Connected) -> Template {
    list(1)
}

fn list(_page: i32) -> Template
{
    let categories: Vec<category::Category> = category::all();
    let mut categories_data = HashMap::<&str, Vec<category::Category>>::new();
    categories_data.insert("categories", categories);

    let data = ContextTemplate {
        categories: categories_data
    };

    Template::render("admin/category/list", data)
}

#[get("/admin/category/new")]
pub fn new(_connected: Connected) -> Template {
    let mut data = HashMap::<&str, bool>::new();

    data.insert("success", true);

    Template::render("admin/category/new", &data)
}

#[post("/admin/category/new", data = "<category>")]
pub fn create(_connected: Connected, category: Form<FormInput>) -> Result<Redirect, String> {
	println!("testsssssssssssssssssssssssssssssssssssssssssss");
    category::create(category);

    Ok(Redirect::to("/admin/category"))
}

#[get("/admin/category/<id>")]
pub fn edit(_connected: Connected, id: i32) -> Template {
    let category_data = category::get_category(id);

    Template::render("admin/category/edit", &category_data)
}

#[post("/admin/category/<id>", data = "<category>")]
pub fn update(_connected: Connected, category: Form<FormInput>, id: i32) -> Template {
    category::update(category, id);
    let category_data = category::get_category(id);

    Template::render("admin/category/edit", &category_data)
}
