use diesel::{
	Queryable,
	sql_types::*,
	pg::Pg,
};
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

use crate::schema::pastes;

#[derive(Serialize, Deserialize, Debug)]
pub struct Paste {
	pub id: String,
	pub filename: Option<String>,
	pub content: Option<String>,
	pub creation_date: NaiveDateTime,
}


impl Queryable<pastes::SqlType, Pg> for Paste {
	type Row = (String, Option<String>, Option<String>, NaiveDateTime);

	fn build(row: Self::Row) -> Self {
		let (id, filename, content, creation_date) = row;
		Paste { id, filename, content, creation_date }
	}
}

impl Queryable<(Text, Nullable<Text>, Timestamp), Pg> for Paste {
	type Row = (String, Option<String>, NaiveDateTime);

	fn build(row: Self::Row) -> Self {
		let (id, filename, creation_date) = row;
		Paste { id, filename, content: None, creation_date }
	}
}

