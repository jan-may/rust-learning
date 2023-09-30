use crate::utils::{
    format_file_size, format_system_time, get_dir_size, get_file_size, get_file_type,
};
use std::fs;
use std::fs::DirEntry;
use std::io;
use std::path::Path;

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

pub fn ls(args: &Vec<String>) {
    let dir = std::env::current_dir().unwrap();
    print_wd(&dir);
    print_table_headers();
    let parent = dir.parent().unwrap();
    if parent.exists() {
        print_table_row(
            "".to_string(),
            "<DIR>",
            "".to_string().as_str(),
            "".to_string().as_str(),
            "..",
        );
    }
    let mut entries = std::fs::read_dir(dir).unwrap();
    // iterate over entries
    while let Some(entry) = entries.next() {
        let entry = entry.unwrap();
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        let metadata = std::fs::metadata(&path).unwrap();
        let file_size = get_file_size(&metadata, &path, &args);
        let file_type = get_file_type(&metadata, &path);
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
