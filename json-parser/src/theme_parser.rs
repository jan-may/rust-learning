use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::collections::HashMap;
use std::fs;

#[derive(Serialize, Deserialize)]
struct Theme {
    colors: HashMap<String, String>,
    space: Vec<i32>,
    font_sizes: Vec<i32>,
    fonts: HashMap<String, String>,
    font_weights: HashMap<String, i32>,
    line_heights: HashMap<String, f32>,
    breakpoints: HashMap<String, String>,
    animation: HashMap<String, String>,
    gradients: HashMap<String, String>,
}

pub fn typed_theme() -> Result<()> {
    let file_path = "data/theme.json".to_owned();
    let data = fs::read_to_string(file_path).expect("Couldn't find or load that file.");
    let theme: Theme = serde_json::from_str(&data)?;
    // Do things just like with any other Rust data structure.
    match theme.colors.get("primary") {
        Some(color) => println!("Primary color is {}", color),
        None => println!("No primary color found"),
    }

    Ok(())
}
