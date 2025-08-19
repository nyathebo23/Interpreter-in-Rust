use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;

use crate::compile_session::CompileSession;
use crate::parser::Parser;
use crate::scanner::display_token;
use crate::scanner::tokenize;
mod scanner;
mod error_handler;
mod parser;
mod tokenizer;
mod compile_session;

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
            let mut errors = false;
            let tokens = tokenize(file_contents, &mut errors);
            display_token(tokens);
            println!("EOF  null"); 
            if errors {
                process::exit(65);
            }
        } ,
        "parse" => {
            let file_contents = file_text(filename);
            let mut errors = false;
            let tokens = tokenize(file_contents, &mut errors);
            if errors {
                process::exit(65);
            }
            let mut parser = Parser::new(&tokens, tokens.len(), 0);
            let express = parser.expression();
            println!("{}", express.to_string());

        },
        "evaluate" => {
            let file_contents = file_text(filename);
            let mut errors = false;

            let tokens = tokenize(file_contents, &mut errors);
            let mut parser = Parser::new(&tokens, tokens.len(), 0);
            let express = parser.expression();
            let result = express.evaluate();
            println!("{}", result.to_str());
        },
        "run" => {
            let file_contents = file_text(filename);
            let mut errors = false;

            let tokens = tokenize(file_contents, &mut errors);
            let parser = Parser::new(&tokens, tokens.len(), 0);
            let mut session = CompileSession::new(parser);
            session.run();

            
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