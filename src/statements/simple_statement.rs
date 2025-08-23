use crate::interpreter::Interpreter;
use crate::parser::declarations::NIL;
use crate::parser::expressions::LiteralExpr;
use crate::scanner::declarations::TokenType;
use crate::statements::{ExprStatement, PrintStatement, VarStatement};


pub fn print_statement(interpreter: &mut Interpreter) -> PrintStatement {
    interpreter.next();
    let expr = interpreter.parser.expression();
    interpreter.check_token(TokenType::SEMICOLON, ";");
    PrintStatement {
        expression: expr
    }
}

pub fn var_statement(interpreter: &mut Interpreter) -> VarStatement {
    interpreter.next();
    let identifier = interpreter.parser.current_token();
    let identifier_str = identifier.lexeme.to_string();
    interpreter.check_token(TokenType::IDENTIFIER, "identifier");
    let token = interpreter.parser.current_token();
    if token.token_type == TokenType::EQUAL {
        interpreter.next();
        let expr = interpreter.parser.expression();
        interpreter.check_token(TokenType::SEMICOLON, ";"); 
        return VarStatement {
            name: identifier_str,
            expression: expr
        };
    }
    else {
        interpreter.check_token(TokenType::SEMICOLON, ";"); 
        return VarStatement {
            name: identifier_str,
            expression: Box::new(LiteralExpr { value:Box::new(NIL) })
        };
    }
}

pub fn expr_statement(interpreter: &mut Interpreter) -> ExprStatement {
    let expr = interpreter.parser.expression();
    if interpreter.parser.current_index + 3 < interpreter.parser.size {
         
    }
    interpreter.check_token(TokenType::SEMICOLON, ";");
    ExprStatement {expression: expr }
}