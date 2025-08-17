use std::env;
use std::fs;
use std::io::{self, Write};

use crate::parser::expression;
use crate::scanner::display_token;
use crate::scanner::tokenize;
mod scanner;
mod error_handler;
mod parser;
mod tokenizer;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

    match command.as_str() {
        "tokenize" => {
            let file_contents = file_text(filename);
            let tokens = tokenize(file_contents);
            display_token(tokens);
            println!("EOF  null"); 
        } ,
        "parse" => {
            let file_contents = file_text(filename);
            let tokens = tokenize(file_contents);
            let mut index: usize = 0;
            let tokens_len = tokens.len();
            let express = expression(&tokens, &mut index, tokens_len);
            println!("{}", express.to_string());
        },
        "evaluate" => {
            let file_contents = file_text(filename);
            let tokens = tokenize(file_contents);
            let mut index: usize = 0;
            let tokens_len = tokens.len();
            let express = expression(&tokens, &mut index, tokens_len);
            let result = express.evaluate();
            println!("{}", result.to_string());
        },
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}


fn file_text(filename: &String) -> String {
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
        String::new()
    });
    file_contents
}