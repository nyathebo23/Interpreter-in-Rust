use std::collections::HashMap;
use std::process;
use crate::error_handler::{handle_error, ErrorType, RUNTIME_ERROR_CODE, SYNTAXIC_ERROR_CODE};
use crate::parser::declarations::{Bool, Number, Object, Str, NIL};
use crate::parser::operators_decl::{UnaryOperator, MAP_COMP_TOKEN_OP, MAP_PLUS_MINUS_OP, MAP_SLASH_STAR_OP};
use crate::scanner::declarations::*;
pub(crate) mod declarations;
mod expressions;
mod utils;
mod operators_decl;
use crate::parser::expressions::*;

pub struct Parser<'a> {
    pub tokens_list: &'a Vec<Token>,
    pub size: usize,
    pub current_index: usize, 
    variables: HashMap<String, Box<dyn Object>>
}

impl Parser<'_> {
    pub fn new(tokens: &Vec<Token>, len: usize, index: usize) -> Parser<'_> {
        Parser {
            tokens_list: tokens,
            size: len,
            current_index: index,
            variables: HashMap::new()
        }
    }

    fn expr_comp_precedence(&mut self, prec_expr: Box<dyn Expression>) -> Box<dyn Expression> 
    {
        let mut left_expr = prec_expr;
        left_expr = self.expr_plus_minus_precedence(left_expr);
        'outer: while self.current_index < self.size {
            let current_token = &self.tokens_list[self.current_index];
            for (token_type, token_op) in MAP_COMP_TOKEN_OP {
                if token_type == current_token.token_type {
                    self.next();
                    let right_expr0 = self.non_binary_expr();
                    let right_expr = self.expr_plus_minus_precedence(right_expr0);
                    left_expr = Box::new(BinaryExpr {
                        operator: token_op,
                        value1: left_expr,
                        value2: right_expr,
                        line: current_token.line
                    });
                    continue 'outer;
                }
            }
            break;
        }
        left_expr
    }

    fn expr_plus_minus_precedence(&mut self, prec_expr: Box<dyn Expression>) -> Box<dyn Expression> 
    {
        let mut left_expr = prec_expr;
        left_expr = self.expr_star_slash_precedence(left_expr);
        'outer: while self.current_index < self.size {
            let current_token = &self.tokens_list[self.current_index];
            for (token_type, token_op) in MAP_PLUS_MINUS_OP {
                if token_type == current_token.token_type {
                    self.next();
                    let right_expr0 = self.non_binary_expr();
                    let right_expr = self.expr_star_slash_precedence(right_expr0);
                    left_expr = Box::new(BinaryExpr {
                        operator: token_op,
                        value1: left_expr,
                        value2: right_expr,
                        line: current_token.line
                    });
                    continue 'outer;
                }
            }
            break;
        }    
        left_expr
    }

    fn expr_star_slash_precedence(&mut self, prec_expr: Box<dyn Expression>) -> Box<dyn Expression>
    {
        let mut left_expr = prec_expr;
        'outer: while  self.current_index < self.size {
            let current_token = &self.tokens_list[self.current_index];
            for (token_type, token_op) in MAP_SLASH_STAR_OP {
                if token_type == current_token.token_type {
                    self.next();
                    let right_expr = self.non_binary_expr();
                    left_expr = Box::new(BinaryExpr {
                        operator: token_op,
                        value1: left_expr,
                        value2: right_expr,
                        line: current_token.line
                    });
                    continue 'outer;
                }
            }
            break;
        }    
        left_expr
    }

    fn non_binary_expr(&mut self) -> Box<dyn Expression> 
    {
        if self.current_index >= self.size {
            self.exit_error(&self.tokens_list[self.current_index].line, "Error: Expect expression.");
        }
        
        let token = &self.tokens_list[self.current_index];
        let expr: Box<dyn Expression>  =  match token.token_type {
            TokenType::LEFTPAREN => {
                self.next();
                let child_expr = self.expression();  
                let expr = GroupExpr {
                    value: child_expr,
                };
                if self.current_index >= self.size || (&self.tokens_list[self.current_index]).token_type != TokenType::RIGHTPAREN  {
                    self.exit_error(&self.tokens_list[self.current_index].line, "Error: Expected character ')'");
                }
                Box::new(expr)
            },
            TokenType::STRING => {
                let token_str = token.literal.clone().unwrap();
                Box::new(LiteralExpr { value: Box::new(Str(token_str)) })
            },
            TokenType::NUMBER => {
                let number = token.literal.clone().unwrap().parse::<f64>().unwrap();
                Box::new(LiteralExpr { value: Box::new(Number(number)) })
            },
            TokenType::IDENTIFIER => {
                match self.variables.get(&token.lexeme.to_string()) {
                    Some(ident) => Box::new(LiteralExpr {value: ident.dyn_clone()} ),
                    None => {
                        handle_error(&token.line, ErrorType::RuntimeError, 
                            format!("Undefined variable '{}'.", token.lexeme).as_str());
                        process::exit(RUNTIME_ERROR_CODE);
                    }
                } 
            },
            TokenType::NIL => Box::new(LiteralExpr { value: Box::new(NIL)}),
            TokenType::TRUE => Box::new(LiteralExpr { value: Box::new(Bool(true)) }),
            TokenType::FALSE => Box::new(LiteralExpr { value: Box::new(Bool(false)) }), 
            TokenType::MINUS => {
                return self.get_unary_expr(token, UnaryOperator::MINUS);
            },
            TokenType::BANG => {
                return self.get_unary_expr(token, UnaryOperator::BANG);
            }
            _ => {
                handle_error(&token.line, ErrorType::SyntacticError, 
                    format!("Error at {0}: Expect expression.", token.lexeme).as_str());
                process::exit(SYNTAXIC_ERROR_CODE);
            }
        };
        self.next();
        expr
    }

    pub fn set_variable(&mut self, identifier: String, val: Box<dyn Object>) {
        self.variables.insert(identifier , val );
    }

    fn exit_error(&self, line: &u32, text: &str) {
        handle_error(line, ErrorType::SyntacticError, text);
        process::exit(SYNTAXIC_ERROR_CODE);  
    }

    pub fn next(&mut self) {
        self.current_index += 1;
    }

    fn get_unary_expr(&mut self, token: &Token, op: UnaryOperator) -> Box<dyn Expression> {
        self.next();
        let child_expr = self.non_binary_expr();      
        let expr = UnaryExpr {
            operator: op,
            value: child_expr,
            line: token.line
        };
        Box::new(expr)
    }

    pub fn expression(&mut self) -> Box<dyn Expression> {
        let start_expr = self.non_binary_expr();
        let expr = self.expr_comp_precedence(start_expr);
        expr
    }

    pub fn current_token(&self) -> &Token {
        &self.tokens_list[self.current_index]
    }
}
