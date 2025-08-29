use std::collections::HashMap;

use crate::class::Class;
use crate::compiler::Compiler;
use crate::error_handler::{handle_error, ErrorType};
use crate::function::Function;
use crate::scanner::declarations::TokenType;
use crate::statements::function_stmt::func_decl;
use crate::statements::ClassDeclStatement;


pub fn class_decl_statement(compiler: &mut Compiler) -> ClassDeclStatement {
    compiler.advance();
    let class_name = compiler.parser.current_token().lexeme.to_string();
    compiler.advance();        
    let mut super_class_name = None;
    compiler.environment.start_class();
    if compiler.parser.current_token().token_type == TokenType::LESS {
        compiler.advance();
        let token = compiler.parser.current_token().clone();
        compiler.parser.check_token(TokenType::IDENTIFIER, "Identifier");
        if token.lexeme.to_string() == class_name {
            handle_error(&token.line, ErrorType::SyntacticError, 
                format!(" Error at {}: A class can't inherit from itself", class_name).as_str());
        }
        super_class_name = Some(token);
        compiler.environment.start_child_class();
    }
    compiler.parser.check_token(TokenType::LEFTBRACE, "{");

    let mut methods = HashMap::new();
    let mut constructor: Option<Function> = None;  
    while compiler.parser.current_token().token_type != TokenType::RIGHTBRACE {
        let funcname = compiler.parser.current_token().lexeme.to_string();
        if funcname == "init" {
            constructor = Some(func_decl(compiler));
            continue;
        }
        methods.insert(funcname, func_decl(compiler,));
    }
    
    compiler.parser.check_token(TokenType::RIGHTBRACE, "}");
    compiler.environment.end_class();
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

