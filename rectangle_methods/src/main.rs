// this file explores how methods can be added to custom structs
#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

// impl 'implements' methods attached to Rectangle struct instances
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // It is ok to have struct fields and methods with the same name.
    // Rust knows the difference between them by whether () is on the end
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }
    // associated function: does not take in 'self'. Usage: [STRUCT]::[ASSOCIATED_FUNCTION]()
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn build_rect(width: u32, height: u32) -> Rectangle {
    Rectangle {
        width,
        height,
    }
}
fn main() {
    let rect1 = build_rect(69, 420);
    let rect2 = build_rect(29, 200);
    let rect3 = build_rect(79, 40);
    let square1 = Rectangle::square(12);
    if rect1.width() {
        println!("non-zero width: {}", rect1.width);
    }
    println!("{}", rect1.area());
    println!("{}", rect1.can_hold(&rect2));
    println!("{}", rect1.can_hold(&rect3));
    println!("{}", square1.area());
}
