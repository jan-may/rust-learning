mod dir;
mod utils;
use crate::dir::{dir};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 || args[1] != "dir" {
        eprintln!("Usage: dir");
        std::process::exit(1);
    }
    dir();
}
