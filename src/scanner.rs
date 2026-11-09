use std::{
    fs::{self, ReadDir},
    ops::ControlFlow::Continue,
    path::Path,
};

pub fn scanner(start_dir: &Path) -> Vec<String> {
    let mut table = Vec::new();
    let result_entries = fs::read_dir(start_dir);

    match result_entries {
        Ok(dir) => {
            for entry in dir {
                let entry = entry.unwrap();
                let file_type = entry.file_type().unwrap();

                if file_type.is_file() {
                    let size = entry.metadata().unwrap().len();
                    table.push(format!("{} {}", entry.path().display(), size));
                    println!("{:?}", entry.path().into_string());
                } else if file_type.is_dir() {
                    table.extend(scanner(&entry.path()));
                }
            }
        }
        Err(_) => println!("XXXX"),
    }

    table
}
