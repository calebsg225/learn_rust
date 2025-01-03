#![allow(unused)]
use std::collections::*;
use std::io::{self, Write};
use std::{cmp::Ordering, fmt::Result, io::Result as IoResult};
mod front_of_house;

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
    pub struct Breakfast {
        pub toast: String,
        seasonal: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal: String::from("Peaches"),
            }
        }
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");

    let mut map = HashMap::new();
    map.insert(1, 2);

    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
}
