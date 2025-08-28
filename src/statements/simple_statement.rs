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
    if identifier_str == "this" {
        handle_error(&identifier.line, ErrorType::SyntacticError, 
            "Error at 'this': variable can't have name 'this'.");
            process::exit(SYNTAXIC_ERROR_CODE)
    }
    interpreter.parser.check_token(TokenType::IDENTIFIER, "identifier");
    let token = interpreter.parser.current_token();
    let line = token.line;
    if token.token_type == TokenType::EQUAL {
        interpreter.parser.next();
        let expr = interpreter.parser.expression();
        interpreter.parser.check_token(TokenType::SEMICOLON, ";"); 
        return VarStatement {
            name: identifier_str,
            expression: expr
        };
    }
    else {
        interpreter.parser.check_token(TokenType::SEMICOLON, ";"); 
        return VarStatement {
            name: identifier_str,
            expression: Box::new(LiteralExpr::new(Box::new(NIL), line))
        };
    }
}

pub fn expr_statement(interpreter: &mut Interpreter) -> ExprStatement {
    let expr = interpreter.parser.expression();
    interpreter.parser.check_token(TokenType::SEMICOLON, ";");
    ExprStatement { expression: expr }
}