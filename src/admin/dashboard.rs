use rocket_contrib::templates::Template;
use std::collections::HashMap;

use crate::session::admin::Connected;


#[get("/admin")]
pub fn index(_connected: Connected) -> Template {
    view(_connected)
}

#[get("/admin/dashboard")]
pub fn dashboard(_connected: Connected) -> Template {
    view(_connected)
}

fn view(connected: Connected) -> Template
{
	let mut context = HashMap::<&str, &str>::new();
	context.insert("connected", "test");

    Template::render("admin/dashboard/index", context)
}
