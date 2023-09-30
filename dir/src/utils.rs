extern crate chrono;
use chrono::NaiveDateTime;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

pub fn format_file_size(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    match size {
        0 => "".to_string(),
        1..=KB => format!("{} B", size),
        KB..=MB => format!("{:.2} KB", size as f64 / KB as f64),
        MB..=GB => format!("{:.2} MB", size as f64 / MB as f64),
        _ => format!("{:.2} GB", size as f64 / GB as f64),
    }
}

pub fn format_system_time(time: SystemTime) -> String {
    time.duration_since(SystemTime::UNIX_EPOCH)
        .map(|duration| {
            let secs = duration.as_secs();
            let human_readable_time = NaiveDateTime::from_timestamp_opt(secs as i64, 0).unwrap();
            human_readable_time.format("%d-%m-%Y %H:%M:%S").to_string()
        })
        .unwrap_or_else(|_| "Unknown".to_string())
}

pub fn get_dir_size(dir: &Path) -> u64 {
    let mut size = 0;

    if dir.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                size += get_dir_size(&path);
            } else {
                size += entry.metadata().unwrap().len();
            }
        }
    } else if dir.is_file() {
        size += dir.metadata().unwrap().len();
    }
    size
}

pub fn get_file_size(metadata: &fs::Metadata, path: &Path, args: &Vec<String>) -> u64 {
    return match (metadata.is_dir(), args.contains(&"-r".to_string())) {
        (true, true) => get_dir_size(&path),
        (true, false) => 0,
        _ => metadata.len(),
    };
}

pub fn get_file_type<'a>(metadata: &'a fs::Metadata, path: &'a Path) -> &'a str {
    return match metadata.file_type().is_dir() {
        true => "<DIR>",
        false => path
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default(),
    };
}

pub fn print_command_description(command: &str, description: &str) {
    println!("  {:<10} {:<10}", command, description);
}
