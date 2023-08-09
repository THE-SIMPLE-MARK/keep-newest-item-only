use std::env;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
	// collect command line arguments
	let args: Vec<String> = env::args().collect();

	// check if the correct number of arguments is provided
	if args.len() != 2 {
		eprintln!("Usage: {} <directory_path>", args[0]);
		return;
	}

	// get the directory path from the command line arguments
	let dir_path = &args[1];
	let dir = Path::new(dir_path);

	// check if the provided path is a valid directory
	if !dir.is_dir() {
		eprintln!("Error: \"{}\" is not a valid directory.", dir_path);
		return;
	}

	// continuously scan the directory and perform operations
	loop {
		// read directory entries
		if let Ok(entries) = fs::read_dir(dir) {
			let mut newest_entry: Option<(String, SystemTime)> = None;

			// find the newest file based on modification timestamps
			for entry in entries {
				if let Ok(entry) = entry {
					if let Ok(metadata) = entry.metadata() {
						let modified_time = metadata.modified().unwrap_or(UNIX_EPOCH);
						let entry_name = entry.file_name().into_string().unwrap_or_else(|_| String::from(""));

						if let Some((_, newest_time)) = newest_entry {
							if modified_time > newest_time {
								newest_entry = Some((entry_name.clone(), modified_time));
							}
						} else {
							newest_entry = Some((entry_name.clone(), modified_time));
						}
					}
				}
			}

			// if a newest file is found, delete other files in the directory
			if let Some((newest_name, _)) = newest_entry {
				for entry in fs::read_dir(dir).unwrap() {
					if let Ok(entry) = entry {
						let entry_name = entry.file_name().into_string().unwrap_or_else(|_| String::from(""));
						if entry_name != newest_name {
							let entry_path = dir.join(&entry_name);
							if entry_path.is_file() {
								// attempt to delete the file
								if fs::remove_file(entry_path.clone()).is_ok() {
									println!("Deleted file \"{}\".", entry_name);
								} else {
									eprintln!("Error deleting file \"{}\".", entry_name);
								}
							}
						}
					}
				}
			}
		}

		// wait for a second before scanning the directory again
		std::thread::sleep(std::time::Duration::from_secs(1));
	}
}