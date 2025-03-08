// Drop Trait
pub struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data {}", self.data);
    }
}

pub fn main() {
    let d1 = CustomSmartPointer {
        data: String::from("foobitch"),
    };
    let d2 = CustomSmartPointer {
        data: String::from("barbitch"),
    };
    println!("CustomSmartPointers created.");
    drop(d1);
    println!("CustomSmartPointer d1 dropped early");
}
