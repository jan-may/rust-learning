use rand::{CryptoRng, Rng};
use std::io::{self, ErrorKind as IoErrorKind};

fn serve_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        // Associated function
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        // super is like .. in file system
        super::serve_order();
    }

    fn cook_order() {}
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // meal.seasonal_fruit = String::from("blueberries"); // error: field `seasonal_fruit` of struct `Breakfast` is private

    // Order a breakfast in the summer with Rye toast
    let order1 = back_of_house::Breakfast::summer("Rye");
    // Order a breakfast in the summer with Rye toast
    let order2 = back_of_house::Breakfast::summer("Rye");
}

mod front_of_house;

pub fn eat_at_restaurant2() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

// use self::front_of_house::hosting;
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant_use() {
    let secret_number = rand::thread_rng().gen_range(1..101);
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
