use walkdir::WalkDir;
use std::fs;
use std::io::Write;

fn main() -> Result<(), std::io::Error> {
	println!("cargo:rerun-if-changed=frontend");
	let frontend_files = WalkDir::new("frontend")
		.into_iter()
		.filter_map(|e| e.ok())
		.filter(|e| e.path().is_file())
		.map(|e| e.path().to_str().unwrap().replace("\\", "/"))
		.map(|filename| format!("(\"{}\", include_str!(\"{}/{}\")), ",
								filename.trim_start_matches("frontend/").to_owned(),
								env!("CARGO_MANIFEST_DIR").replace("\\", "/"),
								filename))
		.collect::<String>();

	let mut frontend_files_file = fs::File::create("src/frontend/frontend_files.array")?;
	frontend_files_file.write_all(b"[")?;
	frontend_files_file.write_all(frontend_files.as_bytes())?;
	frontend_files_file.write_all(b"]")?;

	Ok(())
}