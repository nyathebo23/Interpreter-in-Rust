use std::process;

use crate::error_handler::{handle_error, ErrorType, SYNTAXIC_ERROR_CODE};
use crate::parser::{block_scopes::BlockScopes, declarations::NIL, expressions::LiteralExpr, Parser};
use crate::scanner::declarations::{Token, TokenType};
use crate::statements::*;

pub struct Interpreter<'a> {
    parser: Parser<'a>,
    state: BlockScopes
}

impl Interpreter<'_> {
    
    pub fn new(parser: Parser<'_>) -> Interpreter {
        Interpreter { parser, state: BlockScopes::new() }
    }

    pub fn run(&mut self) {
        let mut stmts: Vec<Box<dyn Statement>> = Vec::new();
        while self.parser.current_index < self.parser.size {
            let token = self.parser.current_token().clone();
            stmts.push(self.statement(&token));
        }
        for stmt in stmts {
            stmt.run(&mut self.state);
        }
    }

    fn print_statement(&mut self) -> PrintStatement {
        self.next();
        let expr = self.parser.expression();
        self.consume(TokenType::SEMICOLON, ";");
        PrintStatement {
            expression: expr
        }
    }

    fn var_statement(&mut self) -> VarStatement {
        self.next();
        let identifier = self.parser.current_token();
        let identifier_str = identifier.lexeme.to_string();
        self.consume(TokenType::IDENTIFIER, "identifier");
        let token = self.parser.current_token();
        if token.token_type == TokenType::EQUAL {
            self.next();
            let expr = self.parser.expression();
            self.consume(TokenType::SEMICOLON, ";"); 
            return VarStatement {
                name: identifier_str,
                expression: expr
            };
        }
        else {
            self.consume(TokenType::SEMICOLON, ";"); 
            return VarStatement {
                name: identifier_str,
                expression: Box::new(LiteralExpr { value:Box::new(NIL) })
            };
        }
    }

    fn block_scope(&mut self) -> BlockStatement {
        let mut stmts: Vec<Box<dyn Statement>> = Vec::new();
        self.next();
        while self.parser.current_index < self.parser.size {
            let token = self.parser.current_token();
            match token.token_type {
                TokenType::VAR => {
                    stmts.push(Box::new(self.var_statement()));
                },
                TokenType::PRINT => {
                    stmts.push(Box::new(self.print_statement()));
                },
                TokenType::LEFTBRACE => {
                    stmts.push(Box::new(self.block_scope()));
                },
                TokenType::RIGHTBRACE => {
                    self.next();
                    return BlockStatement {
                        statements: stmts
                    };
                },
                TokenType::IF => {
                    stmts.push(Box::new(self.if_statement()));
                },
                _ => {
                    let expr = self.parser.expression();
                    self.consume(TokenType::SEMICOLON, ";");
                    stmts.push(Box::new(ExprStatement {expression: expr }));
                } 
            } 
        }

        self.parser.current_index -= 1;
        let last_token = self.parser.current_token();
        handle_error(&last_token.line, ErrorType::SyntacticError, 
            format!("Error at {}: Expect '}}'", last_token.lexeme).as_str());
        process::exit(SYNTAXIC_ERROR_CODE);  
    }

    fn statement(&mut self, token: &Token) -> Box<dyn Statement> {
        match token.token_type {
            TokenType::VAR => {
                Box::new(self.var_statement())
            },
            TokenType::PRINT => {
                Box::new(self.print_statement())
            },
            TokenType::LEFTBRACE => {
                Box::new(self.block_scope())
            },
            TokenType::IF => {
                Box::new(self.if_statement())
            },
            _ => {
                let expr = self.parser.expression();
                self.consume(TokenType::SEMICOLON, ";");
                Box::new(ExprStatement {expression: expr })
            } 
        } 
    }

    fn if_statement(&mut self) -> IfStatement {
        self.next();
        let cond_expr = self.parser.expression();
        let token = self.parser.current_token().clone();
        let if_body = self.statement(&token);
        if self.parser.current_index == self.parser.size {
            return IfStatement {
                condition: cond_expr,
                body: if_body,
                else_if_options: Vec::new(),
                else_statement: None
            };  
        }
        let mut new_token = self.parser.current_token().clone();
        let mut elif_stmts: Vec<PartIfStatement> = Vec::new();
        while new_token.token_type == TokenType::ELSE {
            self.next();
            new_token = self.parser.current_token().clone();

            if new_token.token_type == TokenType::IF {
                self.next();
                let sub_if_cond = self.parser.expression();
                let next_token = self.parser.current_token().clone();
                let sub_if_body = self.statement(&next_token);
                elif_stmts.push(
                    PartIfStatement {
                        condition: sub_if_cond,
                        body: sub_if_body
                    }
                );
                if self.parser.current_index < self.parser.size {
                    new_token = self.parser.current_token().clone();
                }
                continue;
            }
            else {
                let else_stmt = self.statement(&new_token);
                return IfStatement {
                    condition: cond_expr,
                    body: if_body,
                    else_if_options: elif_stmts,
                    else_statement: Some(else_stmt)
                };
            }
        }
        IfStatement {
            condition: cond_expr,
            body: if_body,
            else_if_options: elif_stmts,
            else_statement: None
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