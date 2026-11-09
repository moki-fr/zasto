use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn scanner(start_dir: &Path) -> Vec<String> {
    let mut table = Vec::new();
    let entries = fs::read_dir(start_dir);

    let default_ignore_list: [PathBuf; 3] = [
        PathBuf::from("/proc/"),
        PathBuf::from("C:/Windows/"),
        PathBuf::from("/dev/"),
    ];

    match entries {
        Ok(dirs) => {
            for entry in dirs {
                let entry = entry.unwrap();
                let file_type = entry.file_type().unwrap();

                if file_type.is_file() {
                    // loops all the ignored paths in the list
                    for i in &default_ignore_list {
                        // if it contains
                        if entry.path().starts_with(i) {
                            // then skip
                            continue;
                        }
                    }

                    let size = entry.metadata();

                    match size {
                        Ok(s) => {
                            table.push(format!("{} {}", entry.path().display(), s.len()));
                            //println!("{:?}", entry.path().into_string());
                        }
                        Err(_) => {
                            //
                        }
                    }
                } else if file_type.is_dir() {
                    table.extend(scanner(&entry.path()));
                }
            }
        }
        Err(_) => {
            //
        }
    }

    table
}
