use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use std::env;

pub fn scan(start_dir: &Path) -> HashMap<String, u64> {
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

    let mut table = HashMap::new();

    for entry in jwalk::WalkDir::new(start_dir) {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };

        let path = entry.path();

        if default_ignore_list
            .iter()
            .any(|ignored| path.starts_with(ignored))
        {
            continue;
        }

        if entry.file_type().is_file() {
            if let Ok(metadata) = entry.metadata() {
                table.insert(path.to_string_lossy().into_owned(), metadata.len());
            }
        }
    }

    table
}

pub fn sort_results(
    scan_result: &HashMap<String, u64>,
    scan_output_lenght: usize,
) -> HashMap<&String, &u64> {
    let mut entries: Vec<_> = scan_result.iter().collect();

    entries.sort_unstable_by_key(|&(_, size)| std::cmp::Reverse(*size));

    let mut first_values: HashMap<&String, &u64> = HashMap::new();

    for (path, size) in entries.iter().take(scan_output_lenght) {
        first_values.insert(path, size);
    }

    first_values
}
