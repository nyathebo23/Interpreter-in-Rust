use std::{io::{stderr, Write}, process};

use crate::parser::expressions::Expression;

pub enum ErrorType {
    LexicalError,
    SyntacticError,
    RuntimeError
}

pub const RUNTIME_ERROR_CODE: i32 = 70;
pub const SYNTAXIC_ERROR_CODE: i32 = 65;
pub const LEXICAL_ERROR_CODE: i32 = 65;


pub fn handle_error(line: &u32, error_type: ErrorType, error_text: &str) {
    let mut stderr = stderr();
    match error_type {
        ErrorType::LexicalError => {
            let _ = stderr.write(format!("[line {line}] Error: {error_text}\n").as_bytes());
        },
        ErrorType::RuntimeError => {
            let _ = stderr.write(format!("{error_text}\n[line {line}]\n").as_bytes());
        },
        ErrorType::SyntacticError => {
            let _ = stderr.write(format!("[line {line}] {error_text}\n").as_bytes());

        }
    }
}

pub fn check_this_usage(expression: &Box<dyn Expression>, is_in_class_func: bool) {
    if expression.contains_identifier(&String::from("this")) && !is_in_class_func {
        handle_error(&expression.get_line(), ErrorType::SyntacticError, 
        "Error at 'this': Can't use 'this' outside of a class.");
        process::exit(SYNTAXIC_ERROR_CODE)
    }
}