use std::collections::{HashMap, HashSet};
use std::process;
use std::rc::Rc;

use crate::error_handler::{check_class_keywords_usage, handle_error, ErrorType, SYNTAXIC_ERROR_CODE};
use crate::function::Function;
use crate::interpreter::{Interpreter};
use crate::parser::declarations::{NIL};
use crate::parser::expressions::{Expression, LiteralExpr};
use crate::scanner::declarations::TokenType;
use crate::statements::controlflow_stmts::block_statements;
use crate::statements::simple_statement::var_statement;
use crate::statements::{FunctionDeclStatement, ReturnStatement, Statement}; 


pub fn return_statement(interpreter: &mut Interpreter, is_init_method: bool, 
    is_in_class_func: bool, is_in_superclass: bool) -> ReturnStatement {
    interpreter.parser.next();
    let token = interpreter.parser.current_token();
    if token.token_type == TokenType::SEMICOLON {
        let nil_expr: Box<dyn Expression> = Box::new(LiteralExpr::new(Box::new(NIL), token.line) );
        interpreter.parser.next();
        return ReturnStatement::new(nil_expr);
    }
    if is_init_method {
        let line = interpreter.parser.current_token().line;
        handle_error(&line, ErrorType::SyntacticError, "Error at 'return': Can't return a value from an initializer");
        process::exit(SYNTAXIC_ERROR_CODE);
    }
    let expr: Box<dyn Expression> = interpreter.parser.expression();
    check_class_keywords_usage(&expr, is_in_class_func, is_in_superclass);
    interpreter.parser.check_token(TokenType::SEMICOLON, ";");
    ReturnStatement::new(expr)
}

pub fn block_func_statement(interpreter: &mut Interpreter, func_params: &Vec<String>, 
    is_init_method: bool, is_in_class_func: bool, is_in_superclass: bool) -> Vec<Box<dyn Statement>> {
    let mut stmts: Vec<Box<dyn Statement>> = Vec::new();
    let mut var_stmts_ident: Vec<String> = Vec::new(); 
    while interpreter.parser.current_index < interpreter.parser.size {
        let token = interpreter.parser.current_token();
        let line = token.line;
        match token.token_type {
            TokenType::VAR => {
                let var_stmt = var_statement(interpreter, is_in_class_func, is_in_superclass);
                if func_params.contains(&var_stmt.name) || var_stmts_ident.contains(&var_stmt.name) {
                    handle_error(&line, ErrorType::SyntacticError, 
                        format!("Error at {}: Already a variable with this name in this scope.", var_stmt.name.clone()).as_str());
                        process::exit(SYNTAXIC_ERROR_CODE);
                }
                if var_stmt.expression.contains_identifier(&var_stmt.name) {
                    handle_error(&line, ErrorType::SyntacticError, 
                        format!("Error at {}: Can't read local variable in its own initializer.", var_stmt.name.clone()).as_str());
                        process::exit(SYNTAXIC_ERROR_CODE);
                }
                var_stmts_ident.push(var_stmt.name.clone());
                stmts.push(Box::new(var_stmt));
            },
            TokenType::RIGHTBRACE => {
                interpreter.parser.next();
                return stmts;
            },
            TokenType::FUN => {
                stmts.push(Box::new(func_decl_statement(interpreter, is_in_class_func, is_in_superclass)));
            },
            _ => stmts.append(&mut block_statements(interpreter, token.token_type, true, 
                is_init_method, is_in_class_func, is_in_superclass))
        } 
    }

    interpreter.parser.current_index -= 1;
    let last_token = interpreter.parser.current_token();
    handle_error(&last_token.line, ErrorType::SyntacticError, 
        format!("Error at {}: Expect '}}'", last_token.lexeme).as_str());
    process::exit(SYNTAXIC_ERROR_CODE);  
}

pub fn func_decl(interpreter: &mut Interpreter, is_init_method: bool, is_in_class_func: bool, 
        is_in_superclass: bool) -> Function {
    let ident_str = interpreter.parser.current_token().lexeme.to_string();
    interpreter.parser.next();        
    let mut params: Vec<String> = Vec::new();
    interpreter.parser.check_token(TokenType::LEFTPAREN, "(");
    let mut current_token = interpreter.parser.current_token();
    let line = current_token.line;
    if current_token.token_type != TokenType::RIGHTPAREN {
        loop {
            current_token = interpreter.parser.current_token();
            params.push(current_token.lexeme.to_string());
            interpreter.parser.check_token(TokenType::IDENTIFIER, "Identifier");
            if interpreter.parser.current_token().token_type != TokenType::COMMA {
                break;
            } 
            interpreter.parser.next();
        }
    }
    interpreter.parser.check_token(TokenType::RIGHTPAREN, ")");

    if has_duplicates_elmts(&params, line) {
        process::exit(SYNTAXIC_ERROR_CODE);
    }

    interpreter.parser.check_token(TokenType::LEFTBRACE, "{");
    
    let statements = block_func_statement(interpreter, &params, is_init_method, is_in_class_func, is_in_superclass);
    
    Function {
        name: ident_str.into(),
        params_names: params.into(),
        statements: Rc::new(statements),
        extra_map: HashMap::new()
    }
}

pub fn func_decl_statement(interpreter: &mut Interpreter, is_in_class_func: bool, is_in_superclass: bool) -> FunctionDeclStatement {
    interpreter.parser.next();
    FunctionDeclStatement {
        function_decl: func_decl(interpreter, false, is_in_class_func, is_in_superclass),
    }
}

fn has_duplicates_elmts(vec: &Vec<String>, line: u32) -> bool {
    let mut seen = HashSet::new();
    for item in vec {
        if !seen.insert(item.clone()) {
            handle_error(&line, ErrorType::SyntacticError, 
                format!("Error at {}: Already a variable with this name in this scope.", item.clone()).as_str());
            return true; // Duplicate found
        }
    }
    false
}