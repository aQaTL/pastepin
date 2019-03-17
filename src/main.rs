#![feature(proc_macro_hygiene, decl_macro)]
#![deny(unsafe_code)]

#[macro_use]
extern crate diesel; //Needed for ORM macros

mod utils;
mod schema;
mod models;
mod pastepin;
mod imagepin;
mod frontend;
mod pagination;

use rocket_contrib::databases::database;
use rocket_cors::*;
use rocket::http::Method;

fn clap_app() -> clap::App<'static, 'static> {
	use clap::*;
	App::new(crate_name!())
		.about(crate_description!())
		.version(crate_version!())
		.author(crate_authors!())
		.arg(Arg::with_name("no-frontend")
			.short("f")
			.long("no-frontend")
			.takes_value(false)
			.help("Run without built in frontend"))
}

#[database("pastepin_db")]
pub struct Db(diesel::PgConnection);

fn main() {
	let app = clap_app().get_matches();

	let options = Cors {
		allowed_origins: AllowedOrigins::all(),
		allowed_methods: [Method::Get, Method::Post].into_iter().map(|&m| From::from(m)).collect(),
		allowed_headers: AllowedHeaders::some(&["Accept", "Content-Type", "Cache-Control"]),
		..Default::default()
	};

	//For heroku deploy
	if let Ok(port) = std::env::var("PORT") {
		if port.parse::<u16>().is_ok() {
			std::env::set_var("ROCKET_PORT", port);
		}
	}
	if let Ok(db_url) = std::env::var("DATABASE_URL") {
		std::env::set_var("ROCKET_DATABASES", format!("{{pastepin_db={{url=\"{}\"}}}}", db_url));
	}

	let mut r = rocket::ignite();
	if !app.is_present("no-frontend") {
		r = r.mount("/", frontend::routes())
	}
	r.mount("/", pastepin::routes())
		.mount("/", imagepin::routes())
		.mount("/", catch_all_options_routes())
		.attach(Db::fairing())
		.manage(options)
		.launch();
}
