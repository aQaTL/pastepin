use rocket::{
	Route,
	get, post,
	http::Status,
	request::Form,
};
use rocket_contrib::json::Json;
use crate::Db;
use diesel::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::models::*;
use crate::schema::pastes::dsl::pastes;
use chrono::NaiveDateTime;

pub fn routes() -> Vec<Route> {
	rocket::routes![get_paste, upload, all_pastes_brief]
}

#[get("/p/<paste_id>")]
pub fn get_paste(db: Db, paste_id: i64) -> Json<Paste> {
	Json(pastes
		.find(paste_id)
		.first(&*db)
		.unwrap())
}

#[get("/a?<count>")]
pub fn all_pastes_brief(db: Db, count: Option<u8>) -> Json<Vec<Paste>> {
	use crate::schema::pastes::dsl::*;
	Json(pastes
		.select((id, filename, creation_date))
		.limit(count.unwrap_or(50) as i64)
		.load::<Paste>(&*db)
		.expect("Error loading pastes"))
}


#[post("/u", data = "<form>")]
pub fn upload(db: Db, form: Form<PasteForm>) -> Status {
	let paste = Paste {
		id: 0, //gets ignored when inserting (custom Insertable impl)
		filename: form.filename.clone(),
		content: form.content.clone(),
		creation_date: now(),
	};

	match diesel::insert_into(pastes).values(paste).execute(&*db) {
		Ok(_) => Status::Created,
		Err(err) => {
			eprintln!("Error: {:?}", err);
			Status::InternalServerError
		}
	}
}

fn now() -> NaiveDateTime {
	match SystemTime::now().duration_since(UNIX_EPOCH) {
		Ok(n) => NaiveDateTime::from_timestamp(n.as_secs() as i64, 0),
		Err(_) => panic!("Time went backwards"),
	}
}