fn main() {
    let arg = 5;
    let answer = add_one(arg);

    assert_eq!(6, answer);
    println!("Hello, world! {}", answer);
}

/// Adds one to the number given
///
/// # Examples
///
/// ```
///# use cargo_crate::mainss;
/// let arg = 5;
/// let answer = mainss::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
///
pub fn add_one(x: i32) -> i32 {
    x + 1
}
