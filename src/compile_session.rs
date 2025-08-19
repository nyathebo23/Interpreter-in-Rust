use std::{process};
use crate::{error_handler::{handle_error, ErrorType}, parser::declarations::NIL};
use crate::scanner::declarations::{TokenType};
use crate::parser::{Parser};


pub struct CompileSession<'a> {
    parser: Parser<'a>
}

impl CompileSession <'_> {
    pub fn new(parser: Parser) -> CompileSession<'_> {
        CompileSession {
            parser
        }
    }
    pub fn run(&mut self) {
        while self.parser.current_index < self.parser.size {
            let token = self.parser.current_token();
            match token.token_type {
                TokenType::PRINT => {
                    self.print_statement();
                },
                TokenType::VAR => {
                    self.var_statement();
                },
                _ => {
                    let expr = self.parser.expression();
                    self.consume(TokenType::SEMICOLON, ";");
                    expr.evaluate();
                } 
            } 
        }

    }

    fn print_statement(&mut self) {
        self.next();
        let expr = self.parser.expression();
        self.consume(TokenType::SEMICOLON, ";");
        let res = expr.evaluate();
        println!("{}", res.to_str());
    }

    fn var_statement(&mut self) {
        self.next();
        let identifier = self.parser.current_token().clone();
        self.consume(TokenType::IDENTIFIER, "identifier");
        let token = self.parser.current_token();
        if token.token_type == TokenType::EQUAL {
            self.next();
            let expr = self.parser.expression();
            self.parser.set_variable(identifier.lexeme.to_string(), expr.evaluate());
        }
        else {
            self.parser.set_variable(identifier.lexeme.to_string(), Box::new(NIL));
        }
        self.consume(TokenType::SEMICOLON, ";"); 
    }

    fn next(&mut self) {
        self.parser.next();
    }

    fn consume(&mut self, tokentype: TokenType, lexeme: &str) {
        let token = self.parser.current_token();
        if token.token_type != tokentype {
            handle_error(&token.line, ErrorType::SyntacticError, 
                format!("Error: Expect {}", lexeme).as_str());
            process::exit(65);  
        }  
        self.next();
    }


}