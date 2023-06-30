#![allow(dead_code, unused_variables)]

enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 }, // anonymous struct
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("call");
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    // --snip--
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let localhost = IpAddrKind::V4(127, 0, 0, 1);

    // opional type
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    let x = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y.unwrap_or(0);

    // match
    value_in_cents(Coin::Quarter(UsState::Alaska));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("five: {:?}", five.unwrap());
    println!("six: {:?}", six.unwrap());
    println!("none: {:?}", none.unwrap_or(0));

    // if let
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    } else {
        println!("not three");
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("state quarter from {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,           // None => None
        Some(i) => Some(i + 1), // Some(i) => Some(i + 1)
    }
}
