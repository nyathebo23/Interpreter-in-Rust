use std::process;

use crate::error_handler::{handle_error, ErrorType, SYNTAXIC_ERROR_CODE};
use crate::interpreter::Interpreter;
use crate::parser::declarations::NIL;
use crate::parser::expressions::LiteralExpr;
use crate::scanner::declarations::TokenType;
use crate::statements::{ExprStatement, PrintStatement, VarStatement};


pub fn print_statement(interpreter: &mut Interpreter) -> PrintStatement {
    interpreter.parser.next();
    let expr = interpreter.parser.expression();
    interpreter.parser.check_token(TokenType::SEMICOLON, ";");
    PrintStatement {
        expression: expr
    }
}

pub fn var_statement(interpreter: &mut Interpreter) -> VarStatement {
    interpreter.parser.next();
    let identifier = interpreter.parser.current_token();
    let identifier_str = identifier.lexeme.to_string();
    let line = identifier.line;
    interpreter.parser.check_token(TokenType::IDENTIFIER, "identifier");
    let token = interpreter.parser.current_token();
    if token.token_type == TokenType::EQUAL {
        interpreter.parser.next();
        let expr = interpreter.parser.expression();
        interpreter.parser.check_token(TokenType::SEMICOLON, ";"); 
        if expr.contains_identifier(&identifier_str) {
            handle_error(&line, ErrorType::SyntacticError, 
                format!("Error at {}: Can't read local variable in its own initializer.", identifier_str).as_str());
                process::exit(SYNTAXIC_ERROR_CODE);
        }
        return VarStatement {
            name: identifier_str,
            expression: expr
        };
    }
    else {
        interpreter.parser.check_token(TokenType::SEMICOLON, ";"); 
        return VarStatement {
            name: identifier_str,
            expression: Box::new(LiteralExpr { value:Box::new(NIL) })
        };
    }
}

pub fn expr_statement(interpreter: &mut Interpreter) -> ExprStatement {
    let expr = interpreter.parser.expression();
    interpreter.parser.check_token(TokenType::SEMICOLON, ";");
    ExprStatement {expression: expr }
}