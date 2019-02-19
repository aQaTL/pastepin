use rocket::{
	Route,
	get, post,
};
use rocket_contrib::json::Json;
use crate::Db;
use diesel::prelude::*;

use crate::models::*;

pub fn routes() -> Vec<Route> {
	rocket::routes![get_paste, upload, all_pastes_brief]
}

use super::schema::pastes::dsl::*;

#[get("/p/<paste_id>")]
pub fn get_paste(db: Db, paste_id: String) -> Json<Option<Paste>> {
	Json(pastes
		.find(paste_id)
		.first(&*db)
		.ok())
}

#[get("/a?<count>")]
pub fn all_pastes_brief(db: Db, count: Option<u8>) -> Json<Vec<Paste>> {
	Json(pastes
		.select((id, filename, creation_date))
		.limit(count.unwrap_or(50) as i64)
		.load::<Paste>(&*db)
		.expect("Error loading pastes"))
}

#[post("/u", format = "json", data = "<data>")]
pub fn upload(data: Json<Paste>) -> Json<String> {
	println!("{:?}", data);

	Json(data.filename.to_owned().unwrap().clone())
}