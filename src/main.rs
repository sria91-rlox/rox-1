mod tokens;
mod scanner;

use std::env;
use std::fs;
use std::io;
use std::str;
use scanner::Scanner;
use tokens::{Token, TokenType, Literal};


//TODO igoring implementing error handling for now, naive solution would be global atomic bool?
/*
https://stackoverflow.com/questions/24158114/what-are-the-differences-between-rusts-String-And-str
TODO Use &str instead of String? stack vs heap.
*/

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: rox [script]");
        std::process::exit(64);
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}

fn run_file(filename: &String) {
    println!("{}", filename);
    let contents = fs::read(filename).expect("Something went wrong reading the file");
    //TODO unwrap can cause panics if contents contains invalid UTF-8, rewrite to match?
    run(str::from_utf8(&contents).unwrap().to_string());
    return;
}

fn run_prompt() {
    loop {
        print!(">");
        let mut line = String::new();

        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        run(line);
    }
}

fn run(source: String) {
    let mut scanner = Scanner::new(source);
    let tokens: &Vec<Token> = scanner.scan_tokens();

    for token in tokens {
        println!("{}", token);
    }
}

fn error(line: u32, message: String) {
    report(line, String::from(""), message);
}

fn report(line: u32, location: String, message: String) {
    eprintln!("[line {}] Error {}: {}", line, location, message);
}