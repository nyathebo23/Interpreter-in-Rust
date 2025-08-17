use std::io::{stderr, Write};

pub enum ErrorType {
    LexicalError,
    SyntacticError,
    RuntimeError
}

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
            let _ = stderr.write(format!("[line {line}] Error {error_text}\n").as_bytes());

        }
    }
}