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
}

#[database("pastepin_db")]
pub struct Db(diesel::PgConnection);

fn main() {
	let _app = clap_app().get_matches();

	rocket::ignite()
		.mount("/", pastepin::routes())
		.mount("/", frontend::routes())
		.attach(Db::fairing())
		.launch();
}
