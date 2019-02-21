use rocket::{
	Route,
	get,
	http,
	response::content::Html,
	response::content,
};
use diesel::prelude::*;
use std::collections::HashMap;
use std::path::PathBuf;
use v_htmlescape::escape;

use crate::Db;
use crate::models::Paste;

pub type FrontendFiles = HashMap<&'static str, &'static str>;

lazy_static::lazy_static! {
	pub static ref FRONTEND_FILES: FrontendFiles = include!("frontend_files.array").iter().cloned().collect();
}

pub fn routes() -> Vec<Route> {
	rocket::routes![index, serve_frontend, get_paste]
}

#[get("/")]
fn index() -> content::Html<&'static str> {
	content::Html(FRONTEND_FILES.get("index.html").unwrap())
}

#[get("/<resource..>", rank = 2)]
fn serve_frontend(resource: PathBuf) -> Option<content::Content<&'static str>> {
	let file = FRONTEND_FILES.get(resource.to_str().unwrap())?;
	if let Some(ext) = resource.extension() {
		if let Some(content_type) = http::ContentType::parse_flexible(ext.to_str().unwrap()) {
			return Some(content::Content(content_type, file));
		}
	}
	Some(content::Content(http::ContentType::Plain, file))
}

#[get("/<paste_id>", rank = 1)]
pub fn get_paste(db: Db, paste_id: i64) -> Option<Html<String>> {
	use crate::schema::pastes::dsl::*;

	let paste: Paste = pastes
		.find(paste_id)
		.first::<Paste>(&*db)
		.ok()?;

	let filename_s = paste.filename.unwrap_or_else(|| String::from("pastepin"));
	let content_s = paste.content.unwrap_or_default();

	//TODO Use template engine or load via js instead of this dirty hack
	let page = format!(include!("retrieve_paste.tmpl"), escape(&filename_s), escape(&content_s));
	Some(Html(page))
}