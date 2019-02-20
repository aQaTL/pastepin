use diesel::{
	Queryable,
	sql_types::*,
	pg::Pg,
	insertable::Insertable,
	query_builder::UndecoratedInsertRecord,
	prelude::*,
};
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use rocket_codegen::FromForm;

use crate::schema::pastes;

#[derive(Serialize, Deserialize, Debug)]
pub struct Paste {
	pub id: i64,
	pub filename: Option<String>,
	pub content: Option<String>,
	pub creation_date: NaiveDateTime,
}

impl Queryable<pastes::SqlType, Pg> for Paste {
	type Row = (i64, Option<String>, Option<String>, NaiveDateTime);

	fn build(row: Self::Row) -> Self {
		let (id, filename, content, creation_date) = row;
		Paste { id, filename, content, creation_date }
	}
}

impl Queryable<(BigInt, Nullable<Text>, Timestamp), Pg> for Paste {
	type Row = (i64, Option<String>, NaiveDateTime);

	fn build(row: Self::Row) -> Self {
		let (id, filename, creation_date) = row;
		Paste { id, filename, content: None, creation_date }
	}
}

impl<'a> Insertable<pastes::table> for Paste {
	type Values = <(Option<diesel::dsl::Eq<pastes::filename, String>>,
					Option<diesel::dsl::Eq<pastes::content, String>>,
					Option<diesel::dsl::Eq<pastes::creation_date, NaiveDateTime>>)
	as Insertable<pastes::table>>::Values;

	fn values(self) -> Self::Values {
		(self.filename.map(|x| pastes::filename.eq(x)),
		 self.content.map(|x| pastes::content.eq(x)),
		 Some(pastes::creation_date.eq(self.creation_date))).values()
	}
}

impl<'a> Insertable<pastes::table> for &'a Paste {
	type Values = <(Option<diesel::dsl::Eq<pastes::filename, &'a String>>,
					Option<diesel::dsl::Eq<pastes::content, &'a String>>,
					Option<diesel::dsl::Eq<pastes::creation_date, &'a NaiveDateTime>>)
	as Insertable<pastes::table>>::Values;

	fn values(self) -> Self::Values {
		(self.filename.as_ref().map(|x| pastes::filename.eq(x)),
		 self.content.as_ref().map(|x| pastes::content.eq(x)),
		 Some(pastes::creation_date.eq(&self.creation_date))).values()
	}
}

impl<'a> UndecoratedInsertRecord<pastes::table> for Paste {}

#[derive(FromForm, Serialize, Deserialize, Debug)]
pub struct PasteForm {
	pub filename: Option<String>,
	pub content: Option<String>,
}
