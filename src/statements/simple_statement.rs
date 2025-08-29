
use crate::compiler::Compiler;
use crate::error_handler::{check_class_keywords_usage};
use crate::parser::declarations::NIL;
use crate::parser::expressions::LiteralExpr;
use crate::scanner::declarations::TokenType;
use crate::statements::{ExprStatement, PrintStatement, VarStatement};


pub fn print_statement(compiler: &mut Compiler) -> PrintStatement {
    compiler.advance();
    let expr = compiler.parser.expression();
    check_class_keywords_usage(&expr,  &compiler.environment);
    compiler.parser.check_token(TokenType::SEMICOLON, ";");
    PrintStatement {
        expression: expr
    }
}

pub fn var_statement(compiler: &mut Compiler) -> VarStatement {
    compiler.advance();
    let identifier = compiler.parser.current_token();
    let identifier_str = identifier.lexeme.to_string();
    compiler.parser.check_token(TokenType::IDENTIFIER, "identifier");
    let token = compiler.parser.current_token();
    let line = token.line;
    if token.token_type == TokenType::EQUAL {
        compiler.advance();
        let expr = compiler.parser.expression();
        check_class_keywords_usage(&expr, &compiler.environment);
        compiler.parser.check_token(TokenType::SEMICOLON, ";"); 
        return VarStatement {
            name: identifier_str,
            expression: expr
        };
    }
    else {
        compiler.parser.check_token(TokenType::SEMICOLON, ";"); 
        return VarStatement {
            name: identifier_str,
            expression: Box::new(LiteralExpr::new(Box::new(NIL), line))
        };
    }
}

pub fn expr_statement(compiler: &mut Compiler) -> ExprStatement {
    let expr = compiler.parser.expression();
    check_class_keywords_usage(&expr, &compiler.environment);
    compiler.parser.check_token(TokenType::SEMICOLON, ";");

    ExprStatement { expression: expr }
}