extern crate chrono;
use chrono::NaiveDateTime;
use std::time::SystemTime;

pub fn format_file_size(size: u64) -> String {
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

pub fn format_system_time(time: SystemTime) -> String {
    time.duration_since(SystemTime::UNIX_EPOCH)
        .map(|duration| {
            let secs = duration.as_secs();
            let human_readable_time = NaiveDateTime::from_timestamp_opt(secs as i64, 0).unwrap();
            human_readable_time.format("%d-%m-%Y %H:%M:%S").to_string()
        })
        .unwrap_or_else(|_| "Unknown".to_string())
}
