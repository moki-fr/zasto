use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn scanner(start_dir: &Path) -> Vec<String> {
    let mut table = Vec::new();
    let entries = fs::read_dir(start_dir);

    let default_ignore_list: [PathBuf; 8] = [
        PathBuf::from("C:/Windows/"),
        PathBuf::from("/proc/"),
        PathBuf::from("/dev/"),
        PathBuf::from("/run/"),
        PathBuf::from("/mnt/"),
        PathBuf::from("/sys/"),
        PathBuf::from("/boot/"),
        PathBuf::from("/media/"),
    ];

    match entries {
        Ok(dirs) => {
            for entry in dirs {
                let entry = entry.unwrap();
                let file_type = entry.file_type().unwrap();
                let mut is_ignored: bool = false; // if a file doesnt pass the filters then turn this to true

                if file_type.is_file() {
                    // loops all the ignored paths in the list

                    for i in &default_ignore_list {
                        // if it contains
                        if entry.path().starts_with(i) {
                            // then skip
                            is_ignored = true;
                            break;
                        }
                    }
                    if is_ignored {
                        // skip
                        continue;
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
                    for i in &default_ignore_list {
                        // if it contains
                        if entry.path().starts_with(i) {
                            // then skip
                            is_ignored = true;
                            break;
                        }
                    }

                    if is_ignored {
                        // skip
                        continue;
                    }

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
