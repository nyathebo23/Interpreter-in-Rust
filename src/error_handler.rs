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

pub fn check_class_keywords_usage(expression: &Box<dyn Expression>, is_in_class_func: bool, is_in_superclass: bool) {
    if !is_in_class_func {
        compile_keyword_class_err(expression, "this");
        compile_keyword_class_err(expression, "super");
    }
    else {
        println!("11111111111111 {}", is_in_superclass);
        if !is_in_superclass && expression.contains_identifier(&String::from("super")) {
            handle_error(&expression.get_line(), ErrorType::SyntacticError, 
            "Error at 'super': Can't use 'super' in a class with no superclass");
            process::exit(SYNTAXIC_ERROR_CODE)
        }
    }
}

fn compile_keyword_class_err(expr: &Box<dyn Expression>, keyword: &str) {
    if expr.contains_identifier(&String::from(keyword)) {
        handle_error(&expr.get_line(), ErrorType::SyntacticError, 
        format!("Error at '{}': Can't use '{}' outside of a class.", keyword, keyword).as_str());
        process::exit(SYNTAXIC_ERROR_CODE)
    }    
}