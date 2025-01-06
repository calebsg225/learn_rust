#![allow(unused)]
// lifetimes in rust

// lifetimes neet to be specified for functions or structs that use references

// lifetimes required when a struct holds a reference instead of an owned type
// this ensures that the struct must not outlive the referenced data

use std::fmt::Display;

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() >= s2.len() {
        s1
    } else {
        s2
    }
}

fn longest_with_announcement<'a, T> (
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    if x.len() >= y.len() {
        println!("Announcement: {}", ann);
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");

    {
        let string2 = "xyz";
        let res = longest(string1.as_str(), string2);
        println!("The longest string is {}", res);
    }
}
