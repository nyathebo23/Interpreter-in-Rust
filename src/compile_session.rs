use std::{process};
use crate::{error_handler::{handle_error, ErrorType, SYNTAXIC_ERROR_CODE}, parser::declarations::NIL};
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
                TokenType::LEFTBRACE => {
                    self.block();
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
        let identifier = self.parser.current_token();
        let identifier_str = identifier.lexeme.to_string();
        self.consume(TokenType::IDENTIFIER, "identifier");
        let token = self.parser.current_token();
        if token.token_type == TokenType::EQUAL {
            self.next();
            let expr = self.parser.expression();
            self.parser.set_init_variable(&identifier_str, expr.evaluate());
        }
        else {
            self.parser.set_init_variable(&identifier_str, Box::new(NIL));
        }
        self.consume(TokenType::SEMICOLON, ";"); 
    }

    fn block(&mut self) {
        self.next();
        self.parser.init_block();
        while self.parser.current_index < self.parser.size {
            let token = self.parser.current_token();
            match token.token_type {
                TokenType::VAR => {
                    self.var_statement();
                },
                TokenType::PRINT => {
                    self.print_statement();
                },
                TokenType::LEFTBRACE => {
                    self.block();
                },
                TokenType::RIGHTBRACE => {
                    self.parser.end_block();
                    self.next();
                    break;
                },
                _ => {
                    let expr = self.parser.expression();
                    self.consume(TokenType::SEMICOLON, ";");
                    expr.evaluate();
                } 
            } 
        }
    }

    fn next(&mut self) {
        self.parser.next();
    }

    fn consume(&mut self, tokentype: TokenType, lexeme: &str) {
        let token = self.parser.current_token();
        if token.token_type != tokentype {
            handle_error(&token.line, ErrorType::SyntacticError, 
                format!("Error at {}: Expect {}", token.lexeme, lexeme).as_str());
            process::exit(SYNTAXIC_ERROR_CODE);  
        }  
        self.next();
    }


}