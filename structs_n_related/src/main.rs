// exploring structs in rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username, // shorthand since field name and parameter are the same
        email,
        sign_in_count: 1,
    }
}
fn tuple_structs() {
    // !!! I just made these examples up only to find out THEY'RE THE EXAMPLES USED IN THE RUST
    // BOOK !!!!
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{}", black.0);
    println!("{}", origin.0);
}
fn unit_like_structs() {
    let subject = AlwaysEqual;
    // useful for setting types on things we don't yet have data for (saves memory)
    // example: later go in and make every instance of AlwaysEqual equal to every instance of
    // another type
}
fn main() {
    tuple_structs();
    unit_like_structs();
    let user1 = build_user(
        // in order for one field of User struct to be mutable, the entire User struct instance must be mutable.
        String::from("bobjoe420"),
        String::from("bobjoe420@shitmail.fuck"),
    );
    let mut user2 = User {
        email: String::from("user2email@shitmail.fuck"),
        ..user1
    };
    println!("{}", user2.username);
    user2.username = String::from("Xxbobjoe69420xX");
    let user3 = User {
        email: String::from("user3email@shitmail.fuck"),
        ..user2
    };
    println!("{}", user3.username);
}
