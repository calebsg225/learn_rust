use std::io;

fn main() {

  let p = [0, 1, 2, 3, 4, 5, 6];

  let mut index = String::new();

  io::stdin()
    .read_line(&mut index)
    .expect("Failed to read line");

  let index: usize = index
    .trim()
    .parse()
    .expect("{} is not a nummber");

  // runtime error when trying to access an out of bounds index in an array
  let element = p[index];

  println!("{}: {}", index, element);

  println!("{}", 69%40);

  // char types must be surrounded by single quotes
  // char type is exactly one character long (obviously?)
  // surrounding with double quotes results in a string
  let nauseated_face: char = '🤢';
  println!("{}", nauseated_face);


  let tuple: (i32, &str, char) = (5, "pew", 'p');
  println!("{:?}", tuple);

  let (x, y, _) = tuple;

  println!("x: {}, y: {}", x, y);

  // shadowing tuple
  let tuple = (7, "pew");

  println!("{}, {}", tuple.0, tuple.1);

  println!("{:?}", tuple);

  // array type and length must be declared on creation
  // array length cannot be changed later
  // useful for static data
  let mut a: [i32; 2] = [5, 4];

  println!("{:?}", a);
  
  a[0] += 1;

  println!("{:?}", a);

  /*
  statements vs expressions

  - statements do not return values, only change things
  - expressions reduce to a resulting value
  - functions are statements with an optional ending expression
  - if the last line (expression) does not end with a semicolon, 
    that expression will be returned by the function.
  */

  let foo = { // surrounding with curly braces means statement
    let bar = 4; // statement inside scope of statement
    bar + 3 // ending expression of a statement
  };
  println!("foo: {foo}");
  // foo is 7

  let condition = true;
  let num = if condition { 7 } else { 8 }; // inline if statement
  println!("if statement: {num}");

  println!("{}", sum_two_integers(6, 9));
  println!("{}", sum_two_integers(4, 20));
  println!("{}", sum_two_integers(69, 420));
  println!("{}", sum_two_integers(4, 2));
}

// a function returns an empty tuple () unless otherwise specified or returned

// function with a defined return type
fn sum_two_integers(a: i32, b: i32) -> i32 {
  a + b // in order to return 'a + b', the expression must not end in a semicolon
}