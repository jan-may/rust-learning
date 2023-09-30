use std::path::Path;
use crate::utils::{format_file_size, format_system_time};

fn print_wd(dir: &Path) {
    println!();
    println!(
        "Verzeichnis: {:?}",
        dir.to_str().unwrap().to_string().replace("\\", "/")
    );
    println!();
}

fn print_table_headers() {
    println!(
        "{0: <10} | {1: <10} | {2: <20} | {3: <20} | {4: <22}",
        "size", "extension", "created", "modified", "Filename"
    );
    println!(
        "{0: <10} | {1: <10} | {2: <20} | {3: <20} | {4: <10}",
        "----------", "----------", "--------------------", "--------------------", "----------",
    );
}

fn print_table_row(
    file_size: String,
    file_type: &str,
    created: &str,
    changed: &str,
    file_name: &str,
) {
    println!(
        "{0: <10} | {1: <10} | {2: <20} | {3: <20} | {4: <10}",
        file_size, file_type, created, changed, file_name
    );
}

pub fn dir() {
    let dir = std::env::current_dir().unwrap();
    print_wd(&dir);
    print_table_headers();
    let mut entries = std::fs::read_dir(dir).unwrap();
    // iterate over entries
    while let Some(entry) = entries.next() {
        let entry = entry.unwrap();
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        let metadata = std::fs::metadata(&path).unwrap();
        let file_size = metadata.len();
        let file_type = if metadata.file_type().is_dir() {
            "<DIR>"
        } else {
            path.extension()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default()
        };
        let created = metadata.created().unwrap();
        let changed = metadata.modified().unwrap();
        let formatted_creation_time = format_system_time(created);
        let formatted_changed_time = format_system_time(changed);
        let formatted_file_size = format_file_size(file_size);

        print_table_row(
            formatted_file_size,
            file_type,
            &formatted_creation_time,
            &formatted_changed_time,
            file_name,
        );
    }
}