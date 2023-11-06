mod ls;
mod mkdir;
mod utils;
use crate::ls::ls;
use crate::mkdir::mkdir;
use utils::print_command_description;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let format_string = "{0: <10} {1: <10}";
    if args.len() < 2 {
        eprintln!("Missing command! Available commands:");
        println!();
        print_command_description("ls", "Lists directory contents");
        print_command_description("mkdir", "Creates a new directory at the current location");
        println!();
        return;
    }

    match args[1].as_str() {
        "ls" => ls(&args),
        "mkdir" => mkdir(&std::path::Path::new(args[2].as_str())),
        _ => eprintln!("Unknown command"),
    }
}
