#![allow(unused)]

#[derive(Debug)]

// generally there are considered to be two main categories of testing:
// Unit testing -> test small parts of the code in isolation
// Integration testing -> test your code like external code would, only using the public interface

struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

struct Guess {
    value: i32,
}

impl Guess {
    fn new(value: i32) -> Guess {
        if 1 > value {
            panic!("Guess value must be greater than or equal to 1, got {}.", value);
        }
        if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.", value);
        }
        Guess { value }
    }
}

// 'private' function to test
fn internal_adder(left: usize, right: usize) -> usize {
    left + right
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(a: usize) -> usize {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), String> {
        if add(2,2) == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal 4"))
        }
    }

    #[test]
    fn internal_adder_adds() {
        assert_eq!(internal_adder(3, 7), 10);
    }

    #[test]
    #[ignore]
    fn it_does_not_work() {
        panic!("Teehee, this test fails");
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(add_two(3), 5);
        assert_eq!(add_two(9), 11);
        assert_eq!(add_two(8), 10);
        assert_eq!(add_two(67), 69);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 9,
            height: 7,
        };
        let smaller = Rectangle {
            width: 7,
            height: 2,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 9,
            height: 7,
        };
        let smaller = Rectangle {
            width: 7,
            height: 2,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn guess_too_large() {
        Guess::new(420);
    }

    #[test]
    #[should_panic(expected = "greater than or equal to 1")]
    fn guess_too_small() {
        Guess::new(-69);
    }
}
