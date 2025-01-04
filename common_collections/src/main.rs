#![allow(unused)]

use std::collections::HashMap;

#[derive(Debug)]
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

    if let Some(n) = second {
        println!("{}", n);
    }

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

    let mut scores = HashMap::new();

    scores.insert("Red".to_string(), 69);
    scores.insert("Blue".to_string(), 420);

    println!("{:#?}", scores);

    let team_name = "Yellow".to_string();

    let team_score = scores.get(&team_name).copied().unwrap_or(0);

    println!("{}", team_score);

    scores.insert(team_name, 800);
    // 'team_score' is now invalid, having been moved to the HashMap 'scores'.
    
    for (team, score) in &scores {
        println!("{}: {}", team, score);
    }

    let text = "never gonna give you up, never gonna let you down, never gonna mess around and desert you";

    let mut word_count_in_text = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_count_in_text.entry(word).or_insert(0);
        *count = *count + 1;
    }

    println!("{:#?}", word_count_in_text);
}
