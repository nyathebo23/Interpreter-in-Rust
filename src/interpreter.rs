use std::process;

use crate::error_handler::{handle_error, ErrorType, SYNTAXIC_ERROR_CODE};
use crate::function_manage::clock_declaration;
use crate::parser::{block_scopes::BlockScopes, Parser};
use crate::scanner::declarations::TokenType;
use crate::statements::controlflow_stmts::statement;
use crate::statements::*;

pub struct Interpreter<'a> {
    pub parser: Parser<'a>,
    pub state: BlockScopes
}

impl Interpreter<'_> {
    
    pub fn new(parser: Parser<'_>) -> Interpreter {
        Interpreter { parser, state: BlockScopes::new() }
    }


    pub fn run(&mut self) {
        self.state.define_function(&String::from("clock"), clock_declaration());
        let mut stmts: Vec<Box<dyn Statement>> = Vec::new();
        while self.parser.current_index < self.parser.size {
            stmts.push(statement(self));
        }
        for stmt in stmts {
            stmt.run(&mut self.state);
        }
    }


    pub fn next(&mut self) {
        self.parser.next();
    }

    pub fn check_token(&mut self, tokentype: TokenType, lexeme: &str) {
        let token = self.parser.current_token();
        if token.token_type != tokentype {
            handle_error(&token.line, ErrorType::SyntacticError, 
                format!("Error at '{}': Expect {}", token.lexeme, lexeme).as_str());
            process::exit(SYNTAXIC_ERROR_CODE);  
        }  
        self.next();
    }

}