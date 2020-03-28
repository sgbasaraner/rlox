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

fn run_prompt() {
    let reader = Interface::new("rlox").expect("Couldn't initialize prompt reader.");

    reader.set_prompt("> ").expect("Couldn't set reader prompt.");

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

fn run(source_code: String) {
    let tokens = tokenize(source_code);
    for token in tokens {
        println!("{:?}", token);
    }
}

fn tokenize(source_code: String) -> Vec<Token> {
    unimplemented!();
}

fn error(line: i32, message: String) {
    report(line, String::new(), message);
}

fn report(line: i32, location: String, message: String) {
    eprintln!("[line {}] Error {}: {}", line, location, message);
    unsafe { HAD_ERROR = true; }
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Literal,
    line: i32
}

#[derive(Debug)]
enum Literal {
    Variant1,
    Variant2,
}

#[derive(Debug)]
enum TokenType {
    // Single-character tokens.           
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,           

    // One or two character tokens.     
    Bang, BangEqual, Equal, EqualEqual,
    Greater, GreaterEqual, Less, LessEqual, 

    // Literals.                                     
    Identifier, String, Number,

    // Keywords.                                     
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,    

    EOF
}