use rocket::{
	Route,
	get
};
use std::collections::HashMap;
use rocket::response::content;
use std::path::PathBuf;
use rocket::http;

pub type FrontendFiles = HashMap<&'static str, &'static str>;

lazy_static::lazy_static! {
	pub static ref FRONTEND_FILES: FrontendFiles = include!("frontend_files.array").iter().cloned().collect();
}

pub fn routes() -> Vec<Route> {
	rocket::routes![index, serve_frontend]
}

#[get("/")]
fn index() -> content::Html<&'static str> {
	content::Html(FRONTEND_FILES.get("index.html").unwrap())
}

#[get("/<resource..>", rank=2)]
fn serve_frontend(resource: PathBuf) -> Option<content::Content<&'static str>> {
	let file = FRONTEND_FILES.get(resource.to_str().unwrap())?;
	if let Some(ext) = resource.extension() {
		if let Some(content_type) = http::ContentType::parse_flexible(ext.to_str().unwrap()) {
			return Some(content::Content(content_type, file));
		}
	}
	Some(content::Content(http::ContentType::Plain, file))
}

