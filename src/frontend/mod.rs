use rocket::{
	*,
	Route,
	get,
	http::Method::Get,
	response::content::Html,
	response::content,
	handler::Outcome,
};
use diesel::prelude::*;
use std::collections::HashMap;
use v_htmlescape::escape;

use crate::Db;
use crate::models::Paste;

pub type FrontendFiles = HashMap<&'static str, &'static str>;

lazy_static::lazy_static! {
	pub static ref FRONTEND_FILES: FrontendFiles = include!("frontend_files.array").iter().cloned().collect();
}

pub fn routes() -> Vec<Route> {
	let mut r = rocket::routes![index, get_paste];
	r.push(serve_frontend());
	r
}

#[get("/")]
fn index() -> content::Html<&'static str> {
	content::Html(FRONTEND_FILES.get("index.html").unwrap())
}

#[derive(Clone, Copy)]
pub struct FrontendHandler();

fn serve_frontend() -> Route {
	Route::ranked(1, Get, "/<resource..>", FrontendHandler())
}

impl Handler for FrontendHandler {
	fn handle(&self, req: &Request, data: Data) -> Outcome<'static> {
		let resource = match req.uri().segments().into_path_buf(false) {
			Ok(path) => path,
			Err(_) => return Outcome::forward(data),
		};
		let file = match FRONTEND_FILES.get(resource.to_str().unwrap()) {
			Some(file) => file,
			None => return Outcome::forward(data),
		};
		if let Some(ext) = resource.extension() {
			if let Some(content_type) = http::ContentType::parse_flexible(ext.to_str().unwrap()) {
				return Outcome::from(req, content::Content(content_type, *file));
			}
		}
		Outcome::from(req, content::Content(http::ContentType::Plain, *file))
	}
}


#[get("/<paste_id>", rank = 2)]
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