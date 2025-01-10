use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = minigrep::Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(69);
    });
    
    // .unwrap_or_else() is not needed here because run() on success only returns ().
    if let Err(e) = minigrep::run(config) {
        eprintln!("Problem reading file: {}", e);
        process::exit(420);
    };
}
