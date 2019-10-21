use chrono::NaiveDateTime;
use rocket::request::Form;

use crate::db::database;
use crate::db::post::Post;
use crate::form::comment::FormInput;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Comment {
    id: i32,
    username: String,
    email: String,
    created_at: NaiveDateTime,
    text: String,
    post_id: i32,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommentPost {
    id: i32,
    username: String,
    email: String,
    created_at: NaiveDateTime,
    text: String,
    post_id: i32,
    post: Post,
}

impl Comment {
	pub fn get_by_post(post_id: i32) -> Vec<Comment> {
		let mysql = database::get_connection();

	    let comments: Vec<Comment> =
		    mysql.prep_exec("SELECT id, username, email, created_at, text, post_id from comment WHERE post_id = :post_id",
		    	params!{"post_id" => post_id}
		    ).map(|result| {
		        result.map(|x| x.unwrap()).map(|row| {
		            let (id, username, email, created_at, text, post_id) = mysql::from_row(row);
		            Comment {
		                id: id,
		                username: username,
		                email: email,
		                created_at: created_at,
		                text: text,
		                post_id: post_id
		            }
		        }).collect()
		    }).unwrap();

	    return comments;
	}

	pub fn get(mut _page: i32, number: i32) -> Vec<CommentPost> {
	    let mysql = database::get_connection();

	    _page = _page - 1;
	    let page:i32 = _page * number;

	    let comments: Vec<CommentPost> =
		    mysql.prep_exec("SELECT c.id, c.username, c.email, c.created_at, c.text,
                p.id, p.title, p.note, p.category_id, p.created_at, p.status
                FROM comment AS c
                JOIN post AS p ON p.id = c.post_id
                ORDER BY c.id DESC LIMIT :limit, :offset",
		    	params!{"limit" => page, "offset" => number}
		    ).map(|result| {
		        result.map(|x| x.unwrap()).map(|row| {
                    let (
                        id, username, email, created_at, text, post_id,
                        post_title, post_note, post_category, post_created_at, status
                    ) = mysql::from_row(row);
                    CommentPost {
                        id: id,
                        username: username,
                        email: email,
                        created_at: created_at,
                        text: text,
                        post_id: post_id,
                        post: Post {
                            id: post_id,
                            title: post_title,
                            note: post_note,
                            category_id: post_category,
                            created_at: post_created_at,
                            status: status,
                        }
                    }
		        }).collect()
		    }).unwrap();
	    // println!("{:#?}", comments);

	    return comments;
	}
}

pub fn create(comment: Form<FormInput>, post_id: i32) -> () {
    println!("Database comment : {:?}", comment);

    let datetime = chrono::offset::Local::now();
    let mysql = database::get_connection();
    let mut stmt = mysql.prepare(r"INSERT INTO comment
        (username, email, text, post_id, created_at)
        VALUES
        (:username, :email, :text, :post_id, :created_at)").unwrap();

    stmt.execute(params!{
        "username" => &comment.username,
        "email" => &comment.email,
        "text" => &comment.text,
        "post_id" => &post_id,
        "created_at" => datetime.format("%Y-%m-%d %H:%M:%S").to_string(),
    }).unwrap();
}

pub fn remove(comment_id: i32) -> () {
    let mysql = database::get_connection();
    let mut stmt = mysql.prepare(r"DELETE FROM comment WHERE id = :id").unwrap();

    stmt.execute(params!{
        "id" => comment_id,
    }).unwrap();
}
