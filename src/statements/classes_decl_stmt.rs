use std::process;

use crate::class::Class;
use crate::error_handler::{handle_error, ErrorType, SYNTAXIC_ERROR_CODE};
use crate::function::Function;
use crate::interpreter::Interpreter;
use crate::scanner::declarations::TokenType;
use crate::statements::function_stmt::func_decl;
use crate::statements::ClassDeclStatement;


pub fn class_decl_statement(interpreter: &mut Interpreter) -> ClassDeclStatement {
    interpreter.parser.next();
    let class_name = interpreter.parser.current_token().lexeme.to_string();
    interpreter.parser.next();        
    let mut super_class_name = None;
    if interpreter.parser.current_token().token_type == TokenType::LESS {
        interpreter.parser.next();
        let token = interpreter.parser.current_token().clone();
        interpreter.parser.check_token(TokenType::IDENTIFIER, "Identifier");
        if token.lexeme.to_string() == class_name {
            handle_error(&token.line, ErrorType::SyntacticError, 
                format!(" Error at {}: A class can't inherit from itself", class_name).as_str());
            process::exit(SYNTAXIC_ERROR_CODE);
        }
        super_class_name = Some(token);
    }
    interpreter.parser.check_token(TokenType::LEFTBRACE, "{");

    let mut methods = Vec::new();
    let mut constructor: Option<Function> = None;  
    while interpreter.parser.current_token().token_type != TokenType::RIGHTBRACE {
        let funcname = interpreter.parser.current_token().lexeme.to_string();
        if funcname == "init" {
            constructor = Some(func_decl(interpreter, true, true));
            continue;
        }
        methods.push(func_decl(interpreter, false, true));
    }
    
    interpreter.parser.check_token(TokenType::RIGHTBRACE, "}");

    let class_obj = Class {
        name: class_name,
        methods,
        constructor,
        super_class: None
    };
    ClassDeclStatement {
        class: class_obj,
        super_class_token: super_class_name
    }
}

