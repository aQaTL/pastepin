use crate::{Db, utils, models::*, pagination::{Paginate, Paginated}};

use rocket::{Route, get, post, response::status, http::{ContentType, MediaType, Status}, Data};
use rocket_contrib::{json::{Json, JsonValue}, json};
use rocket_cors::{Guard, Responder};
use diesel::prelude::*;
use multipart::server::{Multipart, save::{SaveResult::*, PartialReason}, MultipartField};
use std::io::Read;
use std::path::PathBuf;

pub fn routes() -> Vec<Route> {
	rocket::routes![get_image, all_images, upload]
}

#[get("/i/<image_id>")]
pub fn get_image(db: Db, cors: Guard, image_id: i64) -> Responder<Option<Image>> {
	use crate::schema::images::dsl::*;
	cors.responder(images
		.find(image_id)
		.first::<Image>(&*db)
		.ok())
}

const DEFAULT_PER_PAGE: i64 = 10;

#[get("/ai?<page>")]
pub fn all_images(db: Db, cors: Guard, page: Option<i64>) -> Responder<Json<Paginated<Image>>> {
	use crate::schema::images::dsl::*;
	cors.responder(Json(images
		.select((id, filename, creation_date))
		.order(id.desc())
		.paginate(page.unwrap_or(1), DEFAULT_PER_PAGE)
		.load_and_count_pages::<Image>(&*db)
		.expect("Unable to load pastes")))
}

const FILE_SIZE_LIMIT: u64 = 10485760;

#[post("/ui", rank = 2, data = "<data>")]
pub fn upload<'a>(db: Db, cors: Guard<'a>, cont_type: &ContentType, data: Data)
				  -> Responder<'a, status::Custom<JsonValue>>
{
	if !cont_type.is_form_data() {
		return cors.responder(status::Custom(
			Status::BadRequest,
			json!({"err": "Content-Type not multipart/form-data".to_string()}),
		));
	}

	let boundary = match cont_type.params().find(|&(k, _)| k == "boundary") {
		Some((_, boundary)) => boundary,
		None => return cors.responder(status::Custom(
			Status::BadRequest,
			json!({"err": "`Content-Type: multipart/form-data` boundary param not provided".to_string()}),
		)),
	};

	let mut multipart = Multipart::with_body(data.open(), boundary);
	let mut ids = Vec::new();
	loop {
		match multipart.read_entry() {
			Ok(Some(entry)) => {
				match process_entry_upload(entry)
					.and_then(|(filename, buf)| insert_entry(&db, filename, buf))
					{
						Ok(id) => ids.push(Ok(id)),
						Err(err_status) => ids.push(Err(err_status.1)),
					}
			}
			Ok(None) => break,
			Err(err) => return cors.responder(status::Custom(
				Status::InternalServerError,
				json!({"err": err.to_string()}))),
		}
	}
	cors.responder(status::Custom(Status::Created, json!(ids)))
}

fn process_entry_upload<R: Read>(mut entry: MultipartField<&mut Multipart<R>>)
								 -> Result<(String, Vec<u8>), status::Custom<JsonValue>>
{
	let filename_from_req = match entry.headers.filename {
		Some(f) => f,
		None => return Err(status::Custom(
			Status::BadRequest,
			json!({"err": "file must have a filename".to_string()}))),
	};
	let filename_pbuf = PathBuf::from(filename_from_req);
	let filename_ = match filename_pbuf.extension()
		.and_then(|ex| ex.to_str())
		.and_then(MediaType::from_extension)
		.map(|media_type| media_type.top() == "image")
		{
			//If it has an extension, it must have a file name
			Some(true) => filename_pbuf.file_name().unwrap().to_str().unwrap().to_string(),
			_ => return Err(status::Custom(
				Status::BadRequest,
				json!({"err": "invalid request".to_string()}))),
		};

	let mut buf = Vec::new();
	match entry.data.save().size_limit(FILE_SIZE_LIMIT).write_to(&mut buf) {
		Full(_) => Ok((filename_, buf)),
		Partial(_, PartialReason::SizeLimit) => Err(status::Custom(
			Status::BadRequest,
			json!({"err": "file too large (max 10 MiB)".to_string()}))),
		Partial(_, PartialReason::CountLimit) => Err(status::Custom(
			Status::BadRequest,
			json!({"err": "too many files".to_string()}))),
		_ => Err(status::Custom(
			Status::InternalServerError,
			json!({"err": "Internal error: {}".to_string()}))),
	}
}

fn insert_entry(db: &Db, filename_: String, data: Vec<u8>) -> Result<i64, status::Custom<JsonValue>> {
	use crate::schema::images::dsl::*;
	match diesel::insert_into(images)
		.values(NewImage { filename: filename_, creation_date: utils::now(), content: data })
		.returning(id)
		.get_result::<i64>(&(db.0))
		{
			Ok(file_id) => Ok(file_id),
			Err(err) => Err(status::Custom(
				Status::InternalServerError,
				json!({"err": err.to_string()}))),
		}
}
