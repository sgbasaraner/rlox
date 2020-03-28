extern crate linefeed;

use linefeed::{Interface, ReadResult};
use std::env;

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

fn run_prompt() {
    let reader = Interface::new("rlox").expect("Couldn't initialize prompt reader.");

    reader.set_prompt("> ").expect("Couldn't set reader prompt.");

    while let ReadResult::Input(input) = reader.read_line().expect("Couldn't read line.") {
        run(input.clone());
        if !input.trim().is_empty() {
            reader.add_history(input);
        }
    }
}

fn run_file(file_name: String) {
    let file_contents = std::fs::read_to_string(file_name).expect("Couldn't read file.");
    run(file_contents);
}

fn run(source_code: String) {
    let tokens = tokenize(source_code);
    for token in tokens {
        println!("{:?}", token);
    }
}

fn tokenize(source_code: String) -> Vec<Token> {
    unimplemented!();
}

#[derive(Debug)]
enum Token {
    Variant1,
    Variant2,
}