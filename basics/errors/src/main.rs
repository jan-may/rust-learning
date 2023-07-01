use std::fs::{self, File};
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
    // panic!("crash and burn");
    // a();

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    // Using closures
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // Shortcuts for Panic on Error: unwrap and expect
    let f = File::open("hello.txt").unwrap(); // unwrap calls panic! on Err
    let f1 = File::open("hello.txt").expect("Failed to open hello.txt"); // expect calls panic! on Err with the message provided

    // Propagating Errors

    //
}

fn a() {
    b();
}

fn b() {
    c(21);
}

fn c(x: i32) {
    if x == 22 {
        panic!("dont pass in 22!");
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    // let mut s = String::new();
    // File::open("hello.txt")?.read_to_string(&mut s)?;
    // Ok(s)

    fs::read_to_string("hello.txt")
}
