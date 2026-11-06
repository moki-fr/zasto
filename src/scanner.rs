use std::{fs, path::Path};

/*
 * Please follow this plan to make the scanner (inspired from the python scanner):
 *
 * if no focused path entered, set it to root
 * create vector where all files come in
 * count unverified files (in case of error)
 * walk through the disk, for each file {
 *  if it's in /proc/ or in C:/Windows/ etc. then Skip
 *  check from ignorelist
 *  try getting info from file, if it can't then add 1 to
 *  add 1 to counted files
 * }
 *
 * Thank you ExpensiveCoal for not following the scanner.py structure.
 *
 */

pub fn scan(focused_path: &Path, scan_output_lenght: u32, ignore_list: &Path) -> Vec<String> {
    let mut table = Vec::new();
    let entries = fs::read_dir(focused_path);

    let matched_entries: fs::ReadDir;

    match entries {
        Ok(tmp) => matched_entries = tmp,
        Err(error) => {
            println!("ERRUH {}", error);
            return Vec::new();
        }
    }

    let mut invalid_file_counter = 0;

    for entry in matched_entries {
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
                    table.extend(scan(&matched_entry.path(), 2, Path::new("/")));
                }
            }
            Err(_) => {
                invalid_file_counter += 1;
            }
        }
    }

    table
}
