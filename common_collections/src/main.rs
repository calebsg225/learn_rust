#![allow(unused)]
#[derive(Debug)] // not required to println!() vectors (unless the contained type requires it)

enum Spreadsheet {
    Int(i32),
    Boolean(bool),
    Text(String),
}

fn main() {
    let mut v = vec![1, 2, 3];

    v.push(5);
    v.push(4);

    println!("{:?}", v);

    let temp = v[3];
    v[3] = v[4];
    v[4] = temp;

    println!("{:?}", v);

    let second: Option<&i32> = v.get(1);

    match second {
        Some(n) => println!("The second element is {}.", n),
        None => println!("There is no second element."),
    }

    let mut v2: Vec<Spreadsheet> = Vec::new();

    v2.push(Spreadsheet::Int(3));

    println!("{:?}", v2);

    let s1 = String::from("foo");
    let s2 = String::from("bar");
    let s3 = format!("{}{}", s1, s2);
    println!("{}, {}", s1, s3);
}
