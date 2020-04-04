mod scanner;
mod grammar;
mod token;
mod parser;

extern crate linefeed;

use linefeed::{Interface, ReadResult};
use std::env;

static mut HAD_ERROR: bool = false;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => run_file(args[1].to_string()),
        1 => run_prompt(),
        _ => {
            println!("Usage: rlox [script]");
            std::process::exit(64);
        },
    }
}

// Meta

fn run_prompt() {
    let reader = Interface::new("rlox").expect("Couldn't initialize prompt reader.");

    reader.set_prompt("rlox> ").expect("Couldn't set reader prompt.");

    while let ReadResult::Input(input) = reader.read_line().expect("Couldn't read line.") {
        run(input.clone());
        if !input.trim().is_empty() {
            reader.add_history(input);
        }
        unsafe { HAD_ERROR = false; }
    }
}

fn run_file(file_name: String) {
    let file_contents = std::fs::read_to_string(file_name).expect("Couldn't read file.");
    run(file_contents);
    unsafe {
        if HAD_ERROR {
            std::process::exit(65);
        }
    }
}

// Interpretation

fn run(source_code: String) {
    let mut scanner = scanner::Scanner::new(source_code);
    let tokens = scanner.scan_tokens();
    for token in tokens {
        println!("{:?}", token);
    }
}

fn error(err: RloxError) {
    report(err.line, err.location, err.message);
}

fn report(line: i32, location: String, message: String) {
    eprintln!("[line {}] Error {}: {}", line, location, message);
    unsafe { HAD_ERROR = true; }
}

pub struct RloxError {
    line: i32,
    message: String,
    location: String
}

impl RloxError {
    pub fn new(line: i32, message: &str, location: &str) -> RloxError {
        RloxError {
            line: line,
            message: message.to_string(),
            location: location.to_string()
        }
    }
}