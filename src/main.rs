#![feature(proc_macro_hygiene, decl_macro)]


#[macro_use] extern crate mysql;
// #[macro_use] extern crate bcrypt;
#[macro_use] extern crate rocket;
// #[macro_use] extern crate rocket_contrib;
// #[macro_use] extern crate chrono;
#[macro_use] extern crate serde_derive;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

mod db;
mod home;
mod form;
mod user;
mod admin;
mod session;

fn main() {
    rocket::ignite()
        .mount("/", routes![
            home::homepage::index,
            home::homepage::page,
            home::homepage::post,
            home::homepage::world,
            home::homepage::comment,
            home::homepage::index_test,
            home::homepage::category,
            user::user::login_page,
            user::user::login,
            user::user::logout,
            admin::dashboard::index,
            admin::dashboard::dashboard,
            admin::post::index,
            admin::post::page,
            admin::post::new,
            admin::post::add,
            admin::post::edit,
            admin::post::update,
            admin::category::index,
            admin::category::new,
            admin::category::create,
            admin::category::edit,
            admin::category::update,
            admin::comment::index,
            admin::comment::page,
            admin::comment::remove,
        ])
        .attach(Template::fairing())
        .mount("/", StaticFiles::from("public"))
        .launch();
}
