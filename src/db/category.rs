use chrono::NaiveDateTime;
use rocket::request::Form;

use crate::form::category::FormInput;
use crate::db::database;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Category {
    id: i32,
    slug: String,
    name: String,
    created_at: NaiveDateTime,
}

pub fn all() -> Vec<Category> {
    let mysql = database::get_connection();

    let categories: Vec<Category> =
        mysql.prep_exec("SELECT id, slug, name, created_at from category", ()).map(|result| {
            result.map(|x| x.unwrap()).map(|row| {
                let (id, slug, name, created_at) = mysql::from_row(row);
                Category {
                    id: id,
                    slug: slug,
                    name: name,
                    created_at: created_at,
                }
            }).collect()
        }).unwrap();
    println!("{:#?}", categories);

    return categories;
}

pub fn get_category(id: i32) -> Category {
    let mysql = database::get_connection();

    for row in mysql.prep_exec("SELECT id, slug, name, created_at from category WHERE id = :id", 
        params!{"id" => id}).unwrap() {

        let (id, slug, name, created_at) = mysql::from_row(row.unwrap());
        // println!("{:?}", row);
        return Category {
            id: id,
            slug: slug,
            name: name,
            created_at: created_at,
        };
    }

    return Category {
        id: 0,
        slug: "slug".to_string(),
        name: "name".to_string(),
        created_at: NaiveDateTime::from_timestamp(0, 42_000_000),
    };
}

pub fn create(category: Form<FormInput>) -> () {
    println!("Database category : {:?}", category);

    let mysql = database::get_connection();
    let mut stmt = mysql.prepare(r"INSERT INTO category (name, slug)
        VALUES (:name, :slug)").unwrap();

    stmt.execute(params!{
        "name" => &category.name,
        "slug" => &category.slug,
    }).unwrap();
}


pub fn update(category: Form<FormInput>, id: i32) -> () {
    // println!("Database category : {:?}", category);

    let mysql = database::get_connection();
    let mut stmt = mysql.prepare(r"UPDATE category
        SET name = :name, slug = :slug
        WHERE id = :id").unwrap();

    stmt.execute(params!{
        "name" => &category.name,
        "slug" => &category.slug,
        "id" => id,
    }).unwrap();
}
