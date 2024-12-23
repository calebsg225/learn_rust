use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
  println!("Guess a number!");

  let secret_number = rand::thread_rng().gen_range(1..=100);

  loop {
    println!("Please input your guess:");

    let mut guess = String::new(); // adding 'mut' after 'let' makes the variable mutable

    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        println!("'{guess}' is not a number!!");
        continue;
      }
    };

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => {
        println!("You win! The number was {secret_number}");
        break;
      }
    }
  }
}
