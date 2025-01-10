use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let ignore = if args.len() > 3 && args[3] == "-ic" {true} else {false};
        Ok(Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
            ignore_case: { env::var("IGNORE_CASE").is_ok() || ignore },
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // since run() returns a Result(T, E), the ? operator can be used.
    let contents = fs::read_to_string(config.file_path)?;

    let lines = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in lines {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut found_lines: Vec<&str> = vec![];

    for line in contents.lines() {
        if line.contains(&query) {
            found_lines.push(line);
        }
    }

    found_lines
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut found_lines: Vec<&str> = vec![];

    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            found_lines.push(line);
        }
    }

    found_lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}
