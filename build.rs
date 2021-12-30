use std::{env, fs, path::PathBuf};

fn main() {
	let cargo_dir =
		PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));
	let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));
	let scss_dir = cargo_dir.join("scss");
	let dir = fs::read_dir(&scss_dir).expect("failed to open scss directory");
	let mut combined_scss = Vec::<String>::new();

	let options = grass::Options::default().load_path(&scss_dir);
	for entry in dir {
		let entry = match entry {
			Ok(entry) => entry,
			_ => continue,
		};
		let path = entry.path();
		// Ensure path ends with .scss and doesn't start with a _
		if path
			.file_name()
			.and_then(|ext| ext.to_str().map(|x| x.starts_with('_')))
			.unwrap_or(false)
			|| !path
				.extension()
				.and_then(|ext| ext.to_str().map(|x| x == "scss"))
				.unwrap_or(false)
		{
			continue;
		}
		let file = fs::read_to_string(&path)
			.unwrap_or_else(|err| panic!("failed to read '{}': {:?}", path.display(), err));
		combined_scss.push(file);
	}
	let compiled_css =
		grass::from_string(combined_scss.join("\n\n"), &options).expect("failed to compile scss");
	fs::write(out_dir.join("style.css"), compiled_css).expect("failed to write style.css");
}
