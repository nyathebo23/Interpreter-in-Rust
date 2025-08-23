use std::process;
use std::rc::Rc;

use crate::error_handler::{handle_error, ErrorType, SYNTAXIC_ERROR_CODE};
use crate::function_manage::Function;
use crate::interpreter::Interpreter;
use crate::parser::declarations::NIL;
use crate::parser::expressions::{Expression, LiteralExpr};
use crate::scanner::declarations::TokenType;
use crate::statements::controlflow_stmts::block_statements;
use crate::statements::simple_statement::var_statement;
use crate::statements::{BlockFuncStatement, FunctionDeclStatement, ReturnStatement, Statement}; 


pub fn return_statement(interpreter: &mut Interpreter) -> ReturnStatement {
    interpreter.next();
    if interpreter.parser.current_token().token_type == TokenType::SEMICOLON {
        let nil_expr: Box<dyn Expression> = Box::new(LiteralExpr{value: Box::new(NIL)});
        interpreter.next();
        return ReturnStatement {
            expression: nil_expr
        };
    }
    let expr: Box<dyn Expression> = interpreter.parser.expression();
    interpreter.check_token(TokenType::SEMICOLON, ";");
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
            TokenType::FUN => {
                stmts.push(Box::new(func_decl_statement(interpreter)));
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

pub fn func_decl_statement(interpreter: &mut Interpreter) -> FunctionDeclStatement {
    interpreter.next();
    let identifier = interpreter.parser.current_token().clone();
    interpreter.next();        
    let mut params: Vec<String> = Vec::new();
    interpreter.check_token(TokenType::LEFTPAREN, "(");
    let mut current_token = interpreter.parser.current_token();
    if current_token.token_type != TokenType::RIGHTPAREN {
        loop {
            current_token = interpreter.parser.current_token();
            params.push(current_token.lexeme.to_string());
            interpreter.check_token(TokenType::IDENTIFIER, "Identifier");
            if interpreter.parser.current_token().token_type != TokenType::COMMA {
                break;
            } 
            interpreter.next();
        }
    }
    interpreter.check_token(TokenType::RIGHTPAREN, ")");
    interpreter.check_token(TokenType::LEFTBRACE, "{");
    let statement = block_func_statement(interpreter);
    let ident_str = identifier.lexeme.to_string();
    let function =     Function {
        name: ident_str.into(),
        params_names: params.into(),
        statement: Rc::new(statement),
    };
    FunctionDeclStatement {
        func_name: function.name.clone(),
        function_decl: function
    }
}