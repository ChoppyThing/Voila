use chrono::NaiveDateTime;
use bcrypt::verify;

use crate::db::database;

#[derive(Debug)]
pub struct User {
	pub id: i32,
	pub username: String,
	pub email: String,
	password: String,
	pub created_at: NaiveDateTime
}

impl User {
	pub fn login(username: &String, password: &String) -> Option<User> {
		let user = crate::db::user::User::find(&username);

		match &user {
			Some(_user_data) => {
				let data = user.unwrap();
				match verify(password, &data.password) {
				    Ok(true) => {
				    	return Some(data);
				    },
				    Ok(false) => {
				    	return None;
				    }
				    _ => {
				    	return None;
				    }
				};
			},
			None => {
				return None;
			}
		}
	}

	fn find(username: &String) -> Option<User> {
		let mysql = database::get_connection();

	    for row in mysql.prep_exec("SELECT id, username, email, password, created_at from user WHERE username = :username", 
	        params!{"username" => username}).unwrap() {

	        let (id, username, email, password, created_at) = mysql::from_row(row.unwrap());

	        return Some(User {
	        	id: id,
	        	username: username,
	        	email: email,
	        	password: password,
	        	created_at: created_at
	        })
	    }

	    None
	}
}
