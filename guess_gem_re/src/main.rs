// remake of guessing game rust
use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    let target = rand::thread_rng().gen_range(1..=100);
    for i in 0..=100 { // '=' means 100 is included in the range
        println!("{}", i);
    }
}
