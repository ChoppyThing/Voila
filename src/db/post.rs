use chrono::NaiveDateTime;
// use mysql::serde::Serialize;
use rocket::request::Form;

use crate::db::database;
use crate::form::post::FormInput;

const PAGE_LIMIT: i32 = 3;
const PAGE_LIMIT_ADMIN: i32 = 15;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Post {
    id: i32,
    title: String,
    note: String,
    category_id: i32,
    created_at: NaiveDateTime,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Posts {
    posts: Vec<Post>,
    count: i32,
    page_list: Vec<i32>,
    page: i32
}

fn get_posts(mut _page: i32, number: i32) -> Vec<Post> {
    let mysql = database::get_connection();

    _page = _page - 1;
    let page:i32 = _page * number;
    // let offset: i32 = page + PAGE_LIMIT;

    let posts: Vec<Post> =
	    mysql.prep_exec("SELECT id, title, note, category_id, created_at from post ORDER BY id DESC LIMIT :limit, :offset",
	    	params!{"limit" => page, "offset" => number}
	    ).map(|result| {
	        result.map(|x| x.unwrap()).map(|row| {
	            let (id, title, note, category_id, created_at) = mysql::from_row(row);
	            Post {
	                id: id,
	                title: title,
	                note: note,
                    category_id: category_id,
	                created_at: created_at,
	            }
	        }).collect()
	    }).unwrap();
    /*println!("{:#?}", posts);*/

    return posts;
}

pub fn get_post(id: i32) -> Post {
    let mysql = database::get_connection();

    for row in mysql.prep_exec("SELECT id, title, note, category_id, created_at from post WHERE id = :id", 
        params!{"id" => id}).unwrap() {

        let (id, title, note, category_id, created_at) = mysql::from_row(row.unwrap());
        // println!("{:?}", row);
        return Post {
            id: id,
            title: title,
            note: note,
            category_id: category_id,
            created_at: created_at,
        };
    }

    return Post {
        id: 0,
        title: "title".to_string(),
        note: "note".to_string(),
        category_id: 0,
        created_at: NaiveDateTime::from_timestamp(0, 42_000_000),
    };
}

fn count() -> i32 {
    let mysql = database::get_connection();

    for row in mysql.prep_exec("SELECT count(*) AS total from post", ()).unwrap() {
    	let total:i32 = mysql::from_row(row.unwrap());

 		return total;
	}

    0
}


/*fn page_list(count: i32, limit: i32) -> Vec<i32> {
    let mut i = 1;
    let mut list = Vec::<i32>::new();

    loop {
    	list.push(i);
    	if i == count / limit {
    		break;
    	} else {
    		i = i + 1;
    	}
    }

    return list;
}*/

fn page_list(count: i32, limit: i32) -> Vec<i32> {
    let mut i = 1;
    let mut counter: i32 = 1;
    let mut list = Vec::<i32>::new();

    loop {
    	if counter >= count {
    		break;
    	}

    	list.push(i);
    	i = i + 1;
		counter = counter + limit;
    }

    return list;
}

pub fn posts(mut _page: i32) -> Posts {
	let count: i32 = count();
	let data = Posts {
		posts: get_posts(_page, PAGE_LIMIT),
		count: count,
		page_list: page_list(count, PAGE_LIMIT),
		page: _page
	};

	data
}

pub fn admin_posts(page: i32) -> Posts {
    let count: i32 = count();
    let data = Posts {
        posts: get_posts(page, PAGE_LIMIT_ADMIN),
        count: count,
        page_list: page_list(count, PAGE_LIMIT_ADMIN),
        page: page
    };

    data
}

pub fn create(post: Form<FormInput>) -> () {
	println!("Database post : {:?}", post);

	let mysql = database::get_connection();
	let mut stmt = mysql.prepare(r"INSERT INTO post
		(title, note, category_id, created_at)
		VALUES
		(:title, :note, :category, :created_at)").unwrap();

	stmt.execute(params!{
        "title" => &post.title,
        "note" => &post.post,
        "category" => &post.category,
        "created_at" => "2019-10-11",
    }).unwrap();
}

pub fn update(post: Form<FormInput>, id: i32) -> () {
    println!("Database post : {:?}", post);

    let mysql = database::get_connection();
    let mut stmt = mysql.prepare(r"UPDATE post
        SET title = :title, note = :note, category_id = :category
        WHERE id = :id").unwrap();

    stmt.execute(params!{
        "title" => &post.title,
        "note" => &post.post,
        "category" => &post.category,
        "id" => id,
    }).unwrap();
}
