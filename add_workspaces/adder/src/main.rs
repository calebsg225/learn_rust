use add_one;
use add_two;
use rand;

fn main() {
    let num = 10;
    println!("foo. {} plus one is {}.", num, add_one::add_one_first(num));
    println!("foo. {} plus two is {}.", num, add_two::add_two_first(num));
}
