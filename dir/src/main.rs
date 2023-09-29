extern crate chrono;

use chrono::NaiveDateTime;
use std::path::Path;
use std::time::SystemTime;

fn format_system_time(time: SystemTime) -> String {
    time.duration_since(SystemTime::UNIX_EPOCH)
        .map(|duration| {
            let secs = duration.as_secs();
            let human_readable_time = NaiveDateTime::from_timestamp_opt(secs as i64, 0).unwrap();
            human_readable_time.format("%d-%m-%Y %H:%M:%S").to_string()
        })
        .unwrap_or_else(|_| "Unknown".to_string())
}

fn format_file_size(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if size < KB {
        format!("{} B", size)
    } else if size < MB {
        format!("{:.2} KB", size as f64 / KB as f64)
    } else if size < GB {
        format!("{:.2} MB", size as f64 / MB as f64)
    } else {
        format!("{:.2} GB", size as f64 / GB as f64)
    }
}

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
        "{0: <10} | {1: <10} | {2: <20} | {3: <21} | {4: <22}",
        "size", "extension", "created", "modified", "Filename"
    );
    println!(
        "{0: <10} | {1: <10} | {2: <20} | {3: <20} | {4: <10}",
        "----------", "----------", "--------------------", "---------------------", "----------",
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 || args[1] != "rustdir" {
        eprintln!("Usage: rustdir");
        std::process::exit(1);
    }
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

        println!(
            "{0: <10} | {1: <10} | {2: <20} | {3: <20?} | {4:?}",
            formatted_file_size,
            file_type,
            formatted_creation_time,
            formatted_changed_time,
            file_name
        );
    }
}
