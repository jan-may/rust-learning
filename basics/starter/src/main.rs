fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    let x = "six";
    println!("The value of x is: {}", x);

    const _MAX_POINTS: u32 = 100_000;

    let _a = 98_222;
    let _b = 0xff;
    let _c = 0o77;
    let _d = 0b1111_0000;
    let _e = b'A';

    // let f: u8 = 256; // #[deny(overflowing_literals)] will cause compile error

    let _sum = 5 + 10;
    let _difference = 95.5 - 4.3;
    let _quotient = 56.7 / 32.2;
    let _remainder = 43 % 5;

    let _t = true;
    let _f: bool = false;

    let tup = ("hallo", 6.4);
    let (_, y) = tup;
    let val = tup.0;
    println!("The value of y is: {}", y);
    println!("The value of val is: {}", val);

    let error_codes = [404, 500, 200];
    let _first = error_codes[0];

    let sum = my_function(10, 12);
    println!("The value of sum is: {}", sum);

    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;
    let _number = if condition { 5 } else { 6 };

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The value of result is: {}", result);

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}", number);
    }
}

fn my_function(x: i32, y: i32) -> i32 {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    x + y
}
