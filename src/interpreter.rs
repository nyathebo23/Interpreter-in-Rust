use std::process;

use crate::error_handler::{handle_error, ErrorType, SYNTAXIC_ERROR_CODE};
use crate::parser::declarations::Bool;
use crate::parser::expressions::Expression;
use crate::parser::{block_scopes::BlockScopes, declarations::NIL, expressions::LiteralExpr, Parser};
use crate::scanner::declarations::TokenType;
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
            stmts.push(self.statement());
        }
        for stmt in stmts {
            stmt.run(&mut self.state);
        }
    }

    fn print_statement(&mut self) -> PrintStatement {
        self.next();
        let expr = self.parser.expression();
        self.check_token(TokenType::SEMICOLON, ";");
        PrintStatement {
            expression: expr
        }
    }

    fn var_statement(&mut self) -> VarStatement {
        self.next();
        let identifier = self.parser.current_token();
        let identifier_str = identifier.lexeme.to_string();
        self.check_token(TokenType::IDENTIFIER, "identifier");
        let token = self.parser.current_token();
        if token.token_type == TokenType::EQUAL {
            self.next();
            let expr = self.parser.expression();
            self.check_token(TokenType::SEMICOLON, ";"); 
            return VarStatement {
                name: identifier_str,
                expression: expr
            };
        }
        else {
            self.check_token(TokenType::SEMICOLON, ";"); 
            return VarStatement {
                name: identifier_str,
                expression: Box::new(LiteralExpr { value:Box::new(NIL) })
            };
        }
    }

    fn expr_statement(&mut self) -> ExprStatement {
        let expr = self.parser.expression();
        self.check_token(TokenType::SEMICOLON, ";");
        ExprStatement {expression: expr }
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
                TokenType::WHILE => {
                    stmts.push(Box::new(self.while_statement()));
                },
                TokenType::FOR => {
                    stmts.push(Box::new(self.for_statement()));
                },
                _ => {
                    stmts.push(Box::new(self.expr_statement()));
                } 
            } 
        }

        self.parser.current_index -= 1;
        let last_token = self.parser.current_token();
        handle_error(&last_token.line, ErrorType::SyntacticError, 
            format!("Error at {}: Expect '}}'", last_token.lexeme).as_str());
        process::exit(SYNTAXIC_ERROR_CODE);  
    }

    fn statement(&mut self) -> Box<dyn Statement> {
        let token = self.parser.current_token();
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
            TokenType::WHILE => {
                Box::new(self.while_statement())
            },
            TokenType::FOR => {
                Box::new(self.for_statement())
            },
            _ => {
                Box::new(self.expr_statement())
            } 
        } 
    }

    fn statement_condition(&mut self) -> Box<dyn Statement> {
        let token = self.parser.current_token();
        match token.token_type {
            TokenType::VAR => {
                handle_error(&token.line, ErrorType::SyntacticError, "Error: Expect expression.");
                process::exit(SYNTAXIC_ERROR_CODE)
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
            TokenType::WHILE => {
                Box::new(self.while_statement())
            },
            TokenType::FOR => {
                Box::new(self.for_statement())
            },
            _ => {
                Box::new(self.expr_statement())
            } 
        } 
    }

    fn if_statement(&mut self) -> IfStatement {
        self.next();
        let cond_expr = self.parser.expression();
        let if_body = self.statement_condition();
        if self.parser.current_index == self.parser.size {
            return IfStatement {
                condition: cond_expr,
                body: if_body,
                else_if_options: Vec::new(),
                else_statement: None
            };  
        }
        let mut new_token = self.parser.current_token();
        let mut elif_stmts: Vec<PartIfStatement> = Vec::new();
        while new_token.token_type == TokenType::ELSE {
            self.next();
            new_token = self.parser.current_token();

            if new_token.token_type == TokenType::IF {
                self.next();
                let sub_if_cond = self.parser.expression();
                let sub_if_body = self.statement_condition();
                elif_stmts.push(
                    PartIfStatement {
                        condition: sub_if_cond,
                        body: sub_if_body
                    }
                );
                if self.parser.current_index < self.parser.size {
                    new_token = self.parser.current_token();
                    continue;
                }
                break;
            }
            else {
                let else_stmt = self.statement();
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

    fn while_statement(&mut self) -> WhileStatement {
        self.next();
        let cond_expr = self.parser.expression();
        let while_body = self.statement_condition();
        WhileStatement {
            condition: cond_expr,
            body: while_body,
        } 
    }

    fn for_statement(&mut self) -> ForStatement {
        self.next();
        self.check_token(TokenType::LEFTPAREN, "(");
        let mut var_decl: Option<VarStatement> = None;
        let token = self.parser.current_token();
        let mut assign_decl: Option<ExprStatement> = None;
        if token.token_type == TokenType::VAR {
            var_decl = Some(self.var_statement());
        }
        else if token.token_type == TokenType::IDENTIFIER {
            assign_decl = Some(ExprStatement {expression: self.parser.expression()});
            self.check_token(TokenType::SEMICOLON, ";");
        }
        
        let mut condition: Box<dyn Expression> = Box::new(LiteralExpr{ value: Box::new(Bool(true)) }); 
        if self.parser.current_token().token_type != TokenType::SEMICOLON {
            condition = self.parser.expression();
        }
        self.check_token(TokenType::SEMICOLON, ";");
        let mut last_instruction: Option<Box<dyn Expression>> = None;
        if self.parser.current_token().token_type != TokenType::RIGHTPAREN {
            last_instruction = Some(self.parser.expression());
        }
        self.check_token(TokenType::RIGHTPAREN, ")");
        let for_body = self.statement_condition();
        ForStatement {
            init_declaration: var_decl,
            init_assignation: assign_decl,
            condition: condition,
            body: for_body,
            last_instruction: last_instruction
        }
    }

    fn next(&mut self) {
        self.parser.next();
    }

    fn check_token(&mut self, tokentype: TokenType, lexeme: &str) {
        let token = self.parser.current_token();
        if token.token_type != tokentype {
            handle_error(&token.line, ErrorType::SyntacticError, 
                format!("Error at '{}': Expect {}", token.lexeme, lexeme).as_str());
            process::exit(SYNTAXIC_ERROR_CODE);  
        }  
        self.next();
    }

}