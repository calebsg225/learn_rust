#![allow(unused)]

use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

const GREETING_FILE_PATH: &str = "hello.txt";

fn main() {
    error_catching_with_closures();
    let res = read_username_from_file_q_operator_fs();
    if let Ok(username) = res {
        println!("username: {}", username)
    };
    // error_catching_within_error_catching();
}

fn it_that_panics() {
    let mut v = Vec::new();
    v.push(6);
    v.push(9);

    v[420];
}

fn error_catching_within_error_catching() {
    let greeting_file_result = File::open(GREETING_FILE_PATH);

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(GREETING_FILE_PATH) {
                Ok(created_file) => created_file,
                Err(failed_creation_error) => {
                    panic!("failed to create file: {:?}", failed_creation_error)
                }
            },
            other_error => {
                panic!("failed to open file: {:?}", other_error)
            }
        },
    };
}

fn error_catching_with_closures() {
    let greeting_file = File::open(GREETING_FILE_PATH).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(GREETING_FILE_PATH).unwrap_or_else(|error| {
                panic!("failed to create file: {:?}", error);
            })
        } else {
            panic!("failed to open file: {:?}", error);
        }
    });
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = match File::open(GREETING_FILE_PATH) {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// same (almost) function as previous function, useing '?' operator
fn read_username_from_file_q_operator() -> Result<String, io::Error> {
    let mut username_file = File::open(GREETING_FILE_PATH)?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_q_operator_chaining() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open(GREETING_FILE_PATH)?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_q_operator_fs() -> Result<String, io::Error> {
    fs::read_to_string(GREETING_FILE_PATH)
}
