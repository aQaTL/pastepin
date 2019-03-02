use rocket::{
	Route,
	get, post,
	request::Form,
	response::status,
	http::Status,
};
use rocket_contrib::{json::{Json, JsonValue}, json};
use crate::Db;
use diesel::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::models::*;
use crate::schema::pastes::dsl::pastes;
use chrono::NaiveDateTime;
use crate::pagination::Paginate;
use diesel::expression::sql_literal::sql;

pub fn routes() -> Vec<Route> {
	rocket::routes![get_paste, upload_generic, upload_json, all_pastes_brief]
}

#[get("/p/<paste_id>")]
pub fn get_paste(db: Db, paste_id: i64) -> Json<Paste> {
	Json(pastes
		.find(paste_id)
		.first::<Paste>(&*db)
		.unwrap())
}

const DEFAULT_PER_PAGE: i64 = 50;

#[get("/a?<page>")]
pub fn all_pastes_brief(db: Db, page: Option<i64>) -> Json<PaginatedPastes> {
	use crate::schema::pastes::dsl::*;
	let page = page.unwrap_or(1);
	let (loaded_pastes, total_pages) = pastes
		.select(
			(id, filename, sql(&format!("SUBSTRING({} from 1 for 150)", content::NAME)), creation_date))
		.order(id.asc())
		.paginate(page, DEFAULT_PER_PAGE)
		.load_and_count_pages::<Paste>(&*db)
		.expect("Unable to load pastes");
	Json(PaginatedPastes { page, total_pages, pastes: loaded_pastes })
}

#[post("/u", rank = 2, data = "<form>")]
pub fn upload_generic(db: Db, form: Form<PasteForm>) -> status::Custom<Option<JsonValue>> {
	upload(db, &form)
}

#[post("/u", rank = 1, data = "<form>", format = "application/json")]
pub fn upload_json(db: Db, form: Json<PasteForm>) -> status::Custom<Option<JsonValue>> {
	upload(db, &form)
}

fn upload(db: Db, form: &PasteForm) -> status::Custom<Option<JsonValue>> {
	let paste = Paste {
		id: 0, //gets ignored when inserting (custom Insertable impl)
		filename: form.filename.clone(),
		content: form.content.clone(),
		creation_date: now(),
	};

	match diesel::insert_into(pastes).values(paste).get_result::<Paste>(&*db) {
		Ok(paste) => status::Custom(Status::Created, Some(json!(
			{"id": paste.id, "creation_date": paste.creation_date}))),
		Err(err) => {
			eprintln!("Error: {:?}", err);
			status::Custom(Status::InternalServerError, None)
		}
	}
}

fn now() -> NaiveDateTime {
	let since_unix = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");
	NaiveDateTime::from_timestamp(since_unix.as_secs() as i64, 0)
}