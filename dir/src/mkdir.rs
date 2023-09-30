use crate::ls::ls;
use std::path::Path;

pub fn mkdir(dir: &Path) {
    std::fs::create_dir_all(dir).unwrap();
    println!("Verzeichnis {:?} erfolgreich angelegt", dir);
}
