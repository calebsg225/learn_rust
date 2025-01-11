#![allow(unused)]
use std::cmp;
use std::thread;
use std::time::Duration;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
    Yellow,
    Green,
}

struct Rectangle {
    width: u32,
    height: u32,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut reds = 0;
        let mut blues = 0;
        let mut yellows = 0;
        let mut greens = 0;

        for shirt in &self.shirts {
            match shirt {
                ShirtColor::Red => reds += 1,
                ShirtColor::Blue => blues += 1,
                ShirtColor::Yellow => yellows += 1,
                ShirtColor::Green => greens += 1,
            }
        }

        let max_count = cmp::max(cmp::max(reds, blues), cmp::max(yellows, greens));
        println!("{} {} {} {} {}", max_count, reds, blues, yellows, greens);

        if max_count == reds {
            return ShirtColor::Red;
        };
        if max_count == blues {
            return ShirtColor::Blue;
        };
        if max_count == yellows {
            return ShirtColor::Yellow;
        };
        ShirtColor::Green
    }
}

fn more_closures() {
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        // thread::sleep(Duration::from_secs(2));
        num
    };

    println!("{}", expensive_closure(3));

    // This closure has no type specified:
    let foo = |x| x;
    // The first time we use 'foo' will define what type 'foo' now expects:
    let s = foo(String::from("bar"));
    // Now 'foo' will only allow Strings.
    println!("{}", s);

    let mut list = vec![1, 2, 3];
    println!("0: {:?}", list);

    let mut b_im = |x| list.push(x);

    // println!("50: {:?}", list);
    // cannot print list here since list is already borrowed as mutable.
    // once a var is borrowed as mutable, more mutable borrows of the same var are not allowed.
    // printing list would borrow as immutable.

    b_im(7);
    b_im(8);
    b_im(9);

    // the closure b_im is allowed to be called more than once since each is using the same
    // borrowed mutable reference to list, therefore no duplicates are made.
    println!("100: {:?}", list);

    // 'move' lets the closure take ownership of 'list'.
    thread::spawn(move || println!("150: {:?}", list))
        .join()
        .unwrap();
    // If the main thread finished before the new thread, list would be dropped and thread would
    // not be able to finish. This is why moving ownership is useful.
    
    let mut list2: [u32; 3] = [
        5,
        6,
        3,
    ];
    let mut sort_ops = vec![];
    let value = String::from("closure called");
    let mut count_calls = 0;

    list2.sort_by_key(|k| {
        // trying to push value without referencing it cannot be called more than once.
        // sort_ops.push(value);
        sort_ops.push(&value);
        count_calls += 1;
        *k
    });
    println!("{:#?}, {}", list2, count_calls);
}

fn main() {
    let store = Inventory {
        shirts: vec![
            ShirtColor::Blue,
            ShirtColor::Green,
            ShirtColor::Blue,
            ShirtColor::Yellow,
            ShirtColor::Red,
            ShirtColor::Blue,
            ShirtColor::Red,
        ],
    };

    let user_pref1 = Some(ShirtColor::Green);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The use with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The use with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
    more_closures();
}
