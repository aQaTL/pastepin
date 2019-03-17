use std::time::{SystemTime, UNIX_EPOCH};
use chrono::NaiveDateTime;

pub fn now() -> NaiveDateTime {
	let since_unix = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");
	NaiveDateTime::from_timestamp(since_unix.as_secs() as i64, 0)
}