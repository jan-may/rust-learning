// "Import" anything public in the parser module
pub mod parser;
pub mod theme_parser;

fn main() {
    // Parse the raw JSON
    let result = parser::untyped_example();
    match result {
        Ok(_result) => (),
        Err(error) => print!("{}", error),
    }

    // Grab JSON file from FS
    let result_from_file = parser::untyped_example_with_file();
    match result_from_file {
        Ok(_result) => (),
        Err(error) => print!("{}", error),
    }

    // typed_example
    parser::typed_example().unwrap_or_else(|error| {
        print!("{}", error);
    });

    // typed_theme
    theme_parser::typed_theme().unwrap_or_else(|error| {
        print!("{}", error);
    });
}
