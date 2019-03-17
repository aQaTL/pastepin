use crate::schema::pastes::dsl::pastes;
use crate::{utils, Db, models::*, pagination::{Paginate, Paginated}};

use rocket::{
	Route,
	get, post,
	request::Form,
	response::status,
	http::Status,
};
use rocket_contrib::{json::{Json, JsonValue}, json};
use rocket_cors::{Guard, Responder};
use diesel::prelude::*;
use diesel::expression::sql_literal::sql;

pub fn routes() -> Vec<Route> {
	rocket::routes![get_paste, upload_generic, upload_json, all_pastes_brief]
}

#[get("/p/<paste_id>")]
pub fn get_paste(db: Db, cors: Guard, paste_id: i64) -> Responder<Json<Paste>> {
	cors.responder(Json(pastes
		.find(paste_id)
		.first::<Paste>(&*db)
		.unwrap()))
}

const DEFAULT_PER_PAGE: i64 = 50;

#[get("/a?<page>")]
pub fn all_pastes_brief(db: Db, cors: Guard, page: Option<i64>) -> Responder<Json<Paginated<Paste>>> {
	use crate::schema::pastes::dsl::*;
	cors.responder(Json(pastes
		.select(
			(id, filename, sql(&format!("SUBSTRING({} from 1 for 150)", content::NAME)), creation_date))
		.order(id.desc())
		.paginate(page.unwrap_or(1), DEFAULT_PER_PAGE)
		.load_and_count_pages::<Paste>(&*db)
		.expect("Unable to load pastes")))
}

#[post("/u", rank = 2, data = "<form>")]
pub fn upload_generic(db: Db, cors: Guard, form: Form<PasteForm>) -> Responder<status::Custom<Option<JsonValue>>> {
	cors.responder(upload(db, &form))
}

#[post("/u", rank = 1, data = "<form>", format = "application/json")]
pub fn upload_json(db: Db, cors: Guard, form: Json<PasteForm>) -> Responder<status::Custom<Option<JsonValue>>> {
	cors.responder(upload(db, &form))
}

fn upload(db: Db, form: &PasteForm) -> status::Custom<Option<JsonValue>> {
	let paste = Paste {
		id: 0, //gets ignored when inserting (custom Insertable impl)
		filename: form.filename.clone(),
		content: form.content.clone(),
		creation_date: utils::now(),
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