use chrono::NaiveDateTime;
// use mysql::serde::Serialize;
use rocket::request::Form;

use crate::db::database;
use crate::form::post::FormInput;

const PAGE_LIMIT: i32 = 3;
const PAGE_LIMIT_ADMIN: i32 = 15;

#[derive(Debug, Serialize, Deserialize)]
enum Status {
    active,
    inactive,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub note: String,
    pub category_id: i32,
    pub status: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Posts {
    posts: Vec<Post>,
    count: i32,
    page_list: Vec<i32>,
    page: i32
}

fn get_posts(mut _page: i32, number: i32, filter: String, is_admin: bool) -> Vec<Post> {
    let mysql = database::get_connection();

    _page = _page - 1;
    let page:i32 = _page * number;
    let (filter_query, params) = get_filter(is_admin, &page, &number, filter);

    let query = format!("SELECT post.id, post.title, post.note, post.category_id, post.created_at, post.status
        from post
        JOIN category ON post.category_id = category.id
        {} 
        ORDER BY id DESC LIMIT :limit, :offset", filter_query);
    let posts: Vec<Post> =
	    mysql.prep_exec(query,
	    	params
	    ).map(|result| {
	        result.map(|x| x.unwrap()).map(|row| {
	            let (id, title, note, category_id, created_at, status): 
                    (i32, String, String, i32, NaiveDateTime, String)
                        = mysql::from_row(row);
	            Post {
	                id: id,
	                title: title,
	                note: note,
                    category_id: category_id,
	                created_at: created_at,
                    status: status,
	            }
	        }).collect()
	    }).unwrap();
    /*println!("{:#?}", posts);*/

    return posts;
}

fn get_filter(is_admin: bool, page: &i32, number: &i32, filter: String) ->  (String, (Vec<(String, mysql::Value)>)) {
    let mut query_filter: String;
    let params: Vec<(String, mysql::Value)>;

    if is_admin {
        query_filter = "WHERE status IN ('active', 'inactive') ".to_string();
    } else {
        query_filter = "WHERE status IN ('active') ".to_string();
    }

    let params: Vec<(String, mysql::Value)>;
    if filter.is_empty() {
        params = params!("limit" => &page, "offset" => &number);
    } else {
        query_filter = format!("{} AND category.slug = :slug ", query_filter);
        params = params!("limit" => &page, "offset" => &number, "slug" => &filter);
    }

    return (query_filter, params);
}

pub fn get_post(id: i32) -> Post {
    let mysql = database::get_connection();

    for row in mysql.prep_exec("SELECT id, title, note, category_id, created_at, status from post WHERE id = :id", 
        params!{"id" => id}).unwrap() {

        let (id, title, note, category_id, created_at, status) = mysql::from_row(row.unwrap());

        return Post {
            id: id,
            title: title,
            note: note,
            category_id: category_id,
            created_at: created_at,
            status: status,
        };
    }

    return Post {
        id: 0,
        title: "title".to_string(),
        note: "note".to_string(),
        category_id: 0,
        status: "inactive".to_string(),
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

pub fn posts(mut _page: i32, filter: String) -> Posts {
	let count: i32 = count();
	let data = Posts {
		posts: get_posts(_page, PAGE_LIMIT, filter, false),
		count: count,
		page_list: page_list(count, PAGE_LIMIT),
		page: _page
	};

	data
}

pub fn admin_posts(page: i32, filter: String) -> Posts {
    let count: i32 = count();
    let data = Posts {
        posts: get_posts(page, PAGE_LIMIT_ADMIN, filter, true),
        count: count,
        page_list: page_list(count, PAGE_LIMIT_ADMIN),
        page: page
    };

    data
}

pub fn create(post: Form<FormInput>) -> () {
    let datetime = chrono::offset::Local::now();
	let mysql = database::get_connection();
	let mut stmt = mysql.prepare(r"INSERT INTO post
		(title, note, category_id, created_at, status)
		VALUES
		(:title, :note, :category, :created_at, :status)").unwrap();

	stmt.execute(params!{
        "title" => &post.title,
        "note" => &post.post,
        "category" => &post.category,
        "status" => &post.status,
        "created_at" => datetime.format("%Y-%m-%d %H:%M:%S").to_string(),
    }).unwrap();
}

pub fn update(post: Form<FormInput>, id: i32) -> () {
    let mysql = database::get_connection();
    let mut stmt = mysql.prepare(r"UPDATE post
        SET title = :title, note = :note, category_id = :category, status = :status
        WHERE id = :id").unwrap();

    stmt.execute(params!{
        "title" => &post.title,
        "note" => &post.post,
        "status" => &post.status,
        "category" => &post.category,
        "id" => id,
    }).unwrap();
}
