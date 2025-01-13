#![allow(unused)]
use std::ops::Deref;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

use crate::List::{Cons, Nil};

fn main() {
    let b: Box<i64> = Box::new(5);
    println!("b = {}", b);

    let list = Box::new(Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Cons(4, Box::new(Cons(5, Box::new(Nil)))))))))));
    
    let x = 5;
    // y holds a reference to x
    let y = &x;
    let z = Box::new(x);
    assert_eq!(5, x);
    // y must be dereferenced in order to be compare to an integer
    assert_eq!(5, *y);
    // z must also be dereferenced as z is a pointer pointing to a value on the heap
    assert_eq!(5, *z);
}
