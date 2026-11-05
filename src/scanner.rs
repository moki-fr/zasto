use std::{fs, path::Path};

pub fn scan(focused_path: &Path, scan_output_lenght: u32, ignore_list: &Path) -> Vec<String> {
    let mut table = Vec::new();
    let entries = fs::read_dir(focused_path).unwrap();

    let mut invalid_file_counter = 0;

    for entry in entries {
        match entry {
            Ok(matched_entry) => {
                let file_type = matched_entry.file_type().unwrap();

                if file_type.is_file() {
                    let size = matched_entry.metadata();

                    match size {
                        Ok(_) => {
                            table.push(format!(
                                "{} {}",
                                matched_entry.path().display(),
                                size.unwrap().len()
                            ));
                        }
                        Err(_) => {
                            println!("Couldn't read a file's metadata");
                        }
                    }
                } else if file_type.is_dir() {
                    table.extend(scan(&matched_entry.path(), 2, Path::new("//")));
                }
            }
            Err(_) => {
                invalid_file_counter += 1;
            }
        }
    }

    println!("Couldnt read {} files", invalid_file_counter);

    table
}
