use std::{io::{stderr, Write}, process};

use crate::{compiler::environment::{ClassType, Environment, FunctionType}, parser::expressions::Expression};

pub enum ErrorType {
    LexicalError,
    SyntacticError,
    RuntimeError
}

pub const RUNTIME_ERROR_CODE: i32 = 70;
pub const SYNTAXIC_ERROR_CODE: i32 = 65;
pub const LEXICAL_ERROR_CODE: i32 = 65;


pub fn handle_error(line: &u32, error_type: ErrorType, error_text: &str) -> ! {
    let mut stderr = stderr();
    match error_type {
        ErrorType::LexicalError => {
            let _ = stderr.write(format!("[line {line}] Error: {error_text}\n").as_bytes());
            process::exit(LEXICAL_ERROR_CODE)
        },
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

pub fn check_class_keywords_usage(expression: &Box<dyn Expression>, environement: &Environment) {
    if environement.current_class == ClassType::NONE {
        compile_keyword_class_err(expression, "this");
        compile_keyword_class_err(expression, "super");
    }
    else {
        if environement.current_class != ClassType::CHILDCLASS && expression.contains_identifier(&String::from("super")) {
            handle_error(&expression.get_line(), ErrorType::SyntacticError, 
            "Error at 'super': Can't use 'super' in a class with no superclass");
        }
    }
}

fn compile_keyword_class_err(expr: &Box<dyn Expression>, keyword: &str) {
    if expr.contains_identifier(&String::from(keyword)) {
        handle_error(&expr.get_line(), ErrorType::SyntacticError, 
        format!("Error at '{}': Can't use '{}' outside of a class.", keyword, keyword).as_str());
    }    
}

pub fn check_var_selfinit(expr: &Box<dyn Expression>, var_name: &String, line: &u32) {
    if expr.contains_identifier(var_name) {
        handle_error(line, ErrorType::SyntacticError, 
            format!("Error at {}: Can't read local variable in its own initializer.", var_name.clone()).as_str());
    }
}

pub fn check_init_classfunc_return(line: &u32, environement: &Environment) {
    if environement.current_function == FunctionType::INITCLASSFUNC {
        handle_error(line, ErrorType::SyntacticError, "Error at 'return': Can't return a value from an initializer");
    }
}

pub fn check_var_redeclaration(block_ident_list: &Vec<String>, var_name: &String, line: &u32) {
    if block_ident_list.contains(var_name) {
        handle_error(&line, ErrorType::SyntacticError, 
        format!("Error at {}: Already a variable with this name in this scope.", var_name.clone()).as_str());
    }
}

pub fn check_return_validity(line: &u32, environement: &Environment) {
    if environement.current_function == FunctionType::NONE {
        handle_error(&line, ErrorType::SyntacticError, "Error at 'return': Can't return from top-level code.");
    }
}