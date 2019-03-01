#![feature(proc_macro_hygiene, decl_macro)]
#![deny(unsafe_code)]

#[macro_use]
extern crate diesel; //Needed for ORM macros

mod schema;
mod models;
mod pastepin;
mod frontend;

use rocket_contrib::databases::database;

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

	let mut r = rocket::ignite();
	if !app.is_present("no-frontend") {
		r = r.mount("/", frontend::routes())
	}
	r.mount("/", pastepin::routes())
		.attach(Db::fairing())
		.launch();
}
