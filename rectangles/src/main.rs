// allows debug printing with println!
#[derive(Debug)]
struct Rectangle {
    length: u64,
    width: u64,
}
fn build_rectangle(length: u64, width: u64) -> Rectangle {
    Rectangle { length, width }
}
// takes in a reference so that ownership is only borrowed
fn calculate_rectangle_area(rect: &Rectangle) -> u64 {
    rect.length * rect.width
}
// println! prints to stdout
// dbg! prints to stderr
// dbg! returns any ownership taken automatically
// dbg! prints out file, row, and column of line it is on
fn main() {
    let scale = 2;
    let rect1 = build_rectangle(dbg!(69 * scale), 420);
    let rect1_area = calculate_rectangle_area(&rect1);
    // & must be prepended to rect1 here only beacuse dbg!'s return value is not being stored back
    // into rect1
    dbg!(&rect1); // #[derive(Debug)] trait must be included in the file for this to work
    println!("{:#?}, {}", rect1, rect1_area);
}
