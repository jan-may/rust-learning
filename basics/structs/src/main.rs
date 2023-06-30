// ignore unused variables
#![allow(unused_variables, dead_code)]

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // method -> self parameter
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // associated function -> no self parameter
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("jan@email.com"),
        username: String::from("jan123"),
        active: true,
        sign_in_count: 1,
    };

    let name = user1.username;
    user1.username = String::from("jan456");

    let user2 = build_user(String::from("sophie@email.com"), String::from("sophie123"));

    let user3 = User {
        email: String::from("peter@email.com"),
        username: String::from("peter123"),
        ..user1 // struct update syntax
    };

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("{:#?}", rect);

    let rect1 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect2 = Rectangle {
        width: 60,
        height: 45,
    };

    let sq = Rectangle::square(3);

    println!("rect can hold rect1: {}", rect.can_hold(&rect1));
    println!("rect can hold rect2: {}", rect.can_hold(&rect2));

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );
}

fn build_user(email: String, username: String) -> User {
    User {
        username: username,
        email, // field init shorthand
        active: true,
        sign_in_count: 1,
    }
}
