mod scanner;
mod grammar;
mod token;
mod parser;
mod eval;

extern crate linefeed;

use linefeed::{Interface, ReadResult};
use eval::Evaluable;
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
        run(input.clone()).err().map(|e| report_error(e));
        if !input.trim().is_empty() {
            reader.add_history(input);
        }
    }
}

fn run_file(file_name: String) {
    let file_contents = std::fs::read_to_string(file_name).expect("Couldn't read file.");
    run(file_contents).err().map(|e| {
        report_error(e);
        std::process::exit(65);
    });
}

// Interpretation

fn run(source_code: String) -> Result<(), RloxError> {
    let mut scanner = scanner::Scanner::new(source_code);
    match scanner.scan_tokens() {
        Ok(tokens) => parser::Parser::new(tokens).parse()
            .and_then(|expr| expr.evaluate())
            .map(|val| println!("{}", val)),
        Err(errs) => {
            for err in errs {
                report_error(err);
            }
            Ok(())
        }
    }
}

fn report_error(err: RloxError) {
    match err.line {
        Some(line) => report(line, err.location, err.message),
        None => report_internal(err.location, err.message)
    }
}

fn report(line: i32, location: String, message: String) {
    eprintln!("[line {}] Error{}: {}", line, location, message);
}

fn report_internal(location: String, message: String) {
    eprintln!("Error {}: {}", location, message);
}

#[derive(Debug)]
pub struct RloxError {
    line: Option<i32>,
    message: String,
    location: String
}

impl RloxError {
    pub fn new(line: i32, message: &str, location: &str) -> RloxError {
        RloxError {
            line: Some(line),
            message: message.to_string(),
            location: location.to_string()
        }
    }

    pub fn internal(message: &str, location: &str) -> RloxError {
        RloxError {
            line: None,
            message: message.to_string(),
            location: location.to_string()
        }
    }
}