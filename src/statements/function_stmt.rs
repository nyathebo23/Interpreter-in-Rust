use std::process;

use crate::error_handler::{handle_error, ErrorType, SYNTAXIC_ERROR_CODE};
use crate::interpreter::Interpreter;
use crate::scanner::declarations::TokenType;
use crate::statements::controlflow_stmts::block_statements;
use crate::statements::simple_statement::var_statement;
use crate::statements::{BlockFuncStatement, ReturnStatement, Statement}; 


pub fn return_statement(interpreter: &mut Interpreter) -> ReturnStatement {
    interpreter.next();
    let expr = interpreter.parser.expression();
    ReturnStatement {
        expression: expr
    }
}

pub fn block_func_statement(interpreter: &mut Interpreter) -> BlockFuncStatement {
    let mut stmts: Vec<Box<dyn Statement>> = Vec::new();
    while interpreter.parser.current_index < interpreter.parser.size {
        let token = interpreter.parser.current_token();
        match token.token_type {
            TokenType::VAR => {
                stmts.push(Box::new(var_statement(interpreter)));
            },
            TokenType::RIGHTBRACE => {
                interpreter.next();
                return BlockFuncStatement {
                    statements: stmts
                };
            },
            _ => stmts.push(block_statements(interpreter, token.token_type))
        } 
    }

    interpreter.parser.current_index -= 1;
    let last_token = interpreter.parser.current_token();
    handle_error(&last_token.line, ErrorType::SyntacticError, 
        format!("Error at {}: Expect '}}'", last_token.lexeme).as_str());
    process::exit(SYNTAXIC_ERROR_CODE);  
}