use rocket::{
	Route,
	get
};

pub fn routes() -> Vec<Route> {
	rocket::routes![index]
}

#[get("/")]
fn index() -> &'static str {
	"Hello from pastepin frontend"
}
