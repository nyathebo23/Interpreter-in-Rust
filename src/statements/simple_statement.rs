
use crate::compiler::Compiler;
use crate::parser::declarations::NIL;
use crate::parser::expressions::LiteralExpr;
use crate::scanner::declarations::TokenType;
use crate::statements::{ExprStatement, PrintStatement, VarStatement};


pub fn print_statement(compiler: &mut Compiler) -> PrintStatement {
    compiler.advance();
    let expr = compiler.parser.expression();
    compiler.environment.check_identifiers(compiler.parser.get_current_expr_identifiers(), expr.get_line());
    compiler.parser.check_token(TokenType::SEMICOLON, ";");
    PrintStatement {
        expression: expr
    }
}

pub fn var_statement(compiler: &mut Compiler) -> VarStatement {
    compiler.advance();
    let identifier = compiler.parser.current_token();
    let identifier_str = identifier.lexeme.to_string();
    let identifier_line = identifier.line;

    compiler.parser.check_token(TokenType::IDENTIFIER, "identifier");
    let token = compiler.parser.current_token();
    if token.token_type == TokenType::EQUAL {
        compiler.advance();
        let expr = compiler.parser.expression();
        compiler.environment.declaration(&identifier_str, &identifier_line, 
            compiler.parser.get_current_expr_identifiers(), expr.get_line());
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
            expression: Box::new(LiteralExpr::new(Box::new(NIL), identifier_line))
        };
    }
}

pub fn expr_statement(compiler: &mut Compiler) -> ExprStatement {
    let expr = compiler.parser.expression();
    compiler.environment.check_identifiers(compiler.parser.get_current_expr_identifiers(), expr.get_line());
    compiler.parser.check_token(TokenType::SEMICOLON, ";");

    ExprStatement { expression: expr }
}