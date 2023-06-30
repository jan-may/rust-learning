fn main() {
    // ----- Ownership rules -----
    // 1. Each value in Rust has a variable that’s called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.
    {
        // s is not valid here, it’s not yet declared
        let _s = String::from("hello"); // s is valid from this point forward
                                        // do stuff with s
    } // this scope is now over, and s is no longer valid

    // ----- Move -----
    let s1 = String::from("hello");
    let _s2 = s1; // s1 is no longer valid

    // ----- Clone -----
    let s1 = String::from("hello");
    let s2 = s1.clone(); // s1 is still valid

    println!("{}, world!", s2);

    // ----- Copy -----
    let x = 5;
    let y = x; // x is still valid

    println!("x = {}, y = {}", x, y);

    // ----- Ownership and Functions -----
    let s = String::from("hello"); // s comes into scope
    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here
                        // println!("{}", s); // error

    let x = 5; // x comes into scope
    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it’s okay to still
                   // use x afterward

    // ----- Return Values and Scope -----
    let s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1
    println!("{}", s1);

    let s2 = String::from("hello"); // s2 comes into scope
    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3
    println!("{}", s3);

    // ----- References and Borrowing -----
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // ----- Mutable References -----
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);

    // ----- Dangling References -----
    // let reference_to_nothing = dangle();
    // println!("{}", reference_to_nothing);

    // ----- The Slice Type -----
    let s = String::from("hello world");
    let _hello = &s[..5]; // first word
    let _world = &s[6..]; // second word
    let word = first_word(&s);
    println!("{}", word);

    let a = [1, 2, 3, 4, 5];
    let _slice = &a[1..3]; // [2, 3]
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("hello"); // some_string comes into scope
    some_string // some_string is returned and
                // moves out to the calling
                // function
}

fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope
    a_string // a_string is returned and moves out to the calling function
}

// ----- References and Borrowing -----
fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

// ----- Mutable References -----
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// ----- Dangling References -----
// fn dangle() -> &String {
//     let s = String::from("hello"); // s is a new String
//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.

// ----- The Slice Type -----
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes(); // convert string to array of bytes

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
