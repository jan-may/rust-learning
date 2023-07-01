use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let _a = [1, 2, 3];
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    {
        let _v2 = vec![1, 2, 3];
    } // v2 goes out of scope and is freed here

    let v3 = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v3[2];

    println!("The third element is {}", third);

    match v3.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."), // v3.get(100) returns None
    }

    for i in &v3 {
        println!("{}", i);
    }

    let mut v4 = vec![100, 32, 57];
    for i in &mut v4 {
        *i += 50;
        println!("{}", i);
    }

    // Using an Enum to Store Multiple Types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];

    match row.get(0) {
        Some(SpreadsheetCell::Int(value)) => println!("The value is {}", value),
        Some(SpreadsheetCell::Float(value)) => println!("The value is {}", value),
        Some(SpreadsheetCell::Text(value)) => println!("The value is {}", value),
        None => println!("There is no value."),
    }

    // Strings are stored as Vec<u8> but are guaranteed to always be valid UTF-8
    let s1 = String::new();
    let s2 = "initial contents";
    let s3 = s2.to_string();
    let s4 = String::from("initial contents");

    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('!');

    let s5 = String::from("Hello, ");
    let s6 = String::from("world!");
    let s7 = s5 + &s6; // Note s5 has been moved here and can no longer be used
    println!("{}", s7);

    // bytes: [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
    // chars: ['न', 'म', 'स', '्', 'त', 'े']
    // graphemes: ['न', 'म', 'स्', 'ते']

    let hello = String::from("Hello");
    let c = hello.chars().nth(0);
    println!("{}", c.unwrap());

    // Indexing into Strings
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{}", s);

    // Iterating Over Strings
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    for g in "नमस्ते".graphemes(true) {
        println!("{}", g);
    }

    // Hash Maps
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");
    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(blue, 10);
    scores.insert(yellow, 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Updating a Hash Map
    scores.insert(String::from("Blue"), 25);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Yellow")).or_insert(70);

    println!("{:?}", scores);

    // Updating a Value Based on the Old Value
    let text = "hello world wonderful world";
    let mut map: HashMap<&str, i32> = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
