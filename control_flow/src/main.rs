fn main() {
  let mut i = 0;
  loop {
    println!("foobar (fubar) :D {}", i);
    i+=69;
    if i >= 420 {break}
  }

  let mut q = 5;
  while q < 534 {
    println!("{}", q);
    q+=60;
  }

  let a = [1, 2, 5, 10, 16, 35, 78, 654, 9853, 125231];
  for el in a {
    println!("{}", el)
  }

  for num in (0..5).rev() {
    println!("{num}")
  }

  let fibs = 1..16;
  for fib in fibs {
    println!("fib {}: {}", fib, fibonacci(fib));
  }
}

fn fibonacci(n: i32) -> i32 {
  if n <= 2 {return 1};
  fibonacci(n - 1) + fibonacci(n - 2)
}
