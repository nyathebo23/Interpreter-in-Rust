use std::{io::{stderr, Write}, process};


pub enum ErrorType {
    SyntacticError,
    RuntimeError
}

pub const RUNTIME_ERROR_CODE: i32 = 70;
pub const SYNTAXIC_ERROR_CODE: i32 = 65;
pub const LEXICAL_ERROR_CODE: i32 = 65;


pub fn handle_error(line: &u32, error_type: ErrorType, error_text: &str) -> ! {
    let mut stderr = stderr();
    match error_type {
        ErrorType::RuntimeError => {
            let _ = stderr.write(format!("{error_text}\n[line {line}]\n").as_bytes());
            process::exit(RUNTIME_ERROR_CODE)
        },
        ErrorType::SyntacticError => {
            let _ = stderr.write(format!("[line {line}] {error_text}\n").as_bytes());
            process::exit(SYNTAXIC_ERROR_CODE)
        }
    }
}
