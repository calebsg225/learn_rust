fn main() {
  // immutable variables' types can be changed via 'shadowing'
  // mutable variables' types cannot be changed

  // BOTH mutable and immutable variables can be shadowed

  // constants CANNOT be changed or shadowed in any way. They are perminantly immutable.
  let mut y = 6;
  println!("{y}");
  y += 5;
  println!("{y}");

  let x = 5;
  let x = x + 2;
  {
    let x = x * 5;
    println!("The value of x in the inner scope is: {x}");
  }
  println!("The value of x in the outer scope is: {x}");

  let spaces = "   ";
  println!("'{spaces}'");
  let spaces = spaces.len();
  println!("'{spaces}'");

  let mut mutable = "pew";
  println!("'{mutable}'");
  mutable = "apeof";
  println!("'{mutable}'");
  let mutable = 5;
  println!("'{mutable}'");
}
