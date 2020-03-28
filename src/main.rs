use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => runFile(args[1].to_string()),
        1 => runPrompt(),
        _ => {
            println!("Usage: rlox [script]");
            std::process::exit(64);
        },
    }
}

fn runPrompt() {
    unimplemented!();
}

fn runFile(file_name: String) {
    let file_contents = std::fs::read_to_string(file_name).expect("Couldn't read file.");
    run(file_contents);
}

fn run(sourceCode: String) {
    unimplemented!();
}