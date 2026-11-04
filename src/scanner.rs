use std::{fs, path::Path};

pub fn scanner(start_dir: &Path) -> Vec<String> {
    let mut table = Vec::new();
    let entries = fs::read_dir(start_dir).unwrap();

    for entry in entries {
        let entry = entry.unwrap();
        let file_type = entry.file_type().unwrap();

        if file_type.is_file() {
            let size = entry.metadata().unwrap().len();
            table.push(format!("{} {}", entry.path().display(), size));

        } else if file_type.is_dir() {
            table.extend(scanner(&entry.path()));

        }
    }

    table
}