// remake of guessing game rust
use rand::Rng;
use std::io;
use std::cmp::Ordering;

const GUESS_MIN_RANGE: i32 = 1;
const GUESS_MAX_RANGE: i32 = 100;

fn main() {
    let target: i32 = rand::thread_rng().gen_range(GUESS_MIN_RANGE..=GUESS_MAX_RANGE);

    loop {
        let mut guess: String = String::new();
        println!("Guess a number between {} and {}", GUESS_MIN_RANGE, GUESS_MAX_RANGE);
        // take user input
        io::stdin()
            .read_line(&mut guess)
            .expect("There was an error taking user input.");
        // make sure user input is an integer
        let guess: i32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("{} could not be converted to an integer.", guess);
                continue;
            }
        };
        // check guess against target
        match guess.cmp(&target) {          // find out why there is an & here
            Ordering::Greater => println!("{} is too high.", guess),
            Ordering::Less => println!("{} is too low.", guess),
            Ordering::Equal => {
                println!("{} the number! You win!", guess);
                break;
            }
        }
    }

}
