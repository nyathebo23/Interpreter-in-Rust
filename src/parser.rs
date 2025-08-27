use std::process;
use std::rc::Rc;
use crate::error_handler::{handle_error, ErrorType,  SYNTAXIC_ERROR_CODE};
use crate::parser::declarations::{Bool, Number, Str, NIL};
use crate::parser::operators_decl::{operators_priority_list, OpChainPriority, UnaryOperator};
use crate::scanner::declarations::*;
pub(crate) mod declarations;
pub mod expressions;
pub mod operators_decl;
use crate::parser::expressions::*;

pub struct Parser<'a> {
    pub tokens_list: &'a Vec<Token>,
    pub size: usize,
    pub current_index: usize, 
    op_priority_list: Rc<OpChainPriority>,
}

impl Parser<'_> {
    pub fn new(tokens: &Vec<Token>, index: usize) -> Parser<'_> {
        Parser {
            tokens_list: tokens,
            size: tokens.len(),
            current_index: index,
            op_priority_list: operators_priority_list().into(),
        }
    }

    fn get_expr_op_priority(&mut self, prec_expr: Box<dyn Expression>, operators_list: &OpChainPriority) -> Box<dyn Expression> {
        match operators_list {
            OpChainPriority::Cons(map_operators, next_map) => {
                let mut left_expr = prec_expr;
                left_expr = self.get_expr_op_priority(left_expr, next_map);
                'outer: while self.current_index < self.size {
                    let current_token = &self.tokens_list[self.current_index];
                    for (token_type, token_op) in map_operators.iter() {
                        if *token_type == current_token.token_type {
                            self.next();
                            let right_expr0 = self.non_binary_expr();
                            let right_expr = self.get_expr_op_priority(right_expr0, next_map);
                            left_expr = Box::new( BinaryExpr::new(*token_op, left_expr, right_expr, current_token.line));
                            continue 'outer;
                        }
                    }
                    break;
                }
                left_expr 
            },
            OpChainPriority::Nil => {
                prec_expr
            }
        }
    }

    fn simple_expression(&mut self) -> Box<dyn Expression> {
        if self.current_index >= self.size {
            self.exit_error(&self.tokens_list[self.current_index].line, "Error: Expect expression.");
        }
        let token = &self.tokens_list[self.current_index];
        let expr: Box<dyn Expression>  =  match token.token_type {
            TokenType::IDENTIFIER => Box::new(
                IdentifierExpr::new(token.lexeme.to_string(), None, token.line)
            ),
            TokenType::LEFTPAREN => {
                self.next();
                let expr = GroupExpr::new(self.expression(), token.line);
                self.check_token_valid(TokenType::RIGHTPAREN, ")");
                Box::new(expr)
            },
            TokenType::STRING => {
                let token_str = token.literal.clone().unwrap();
                Box::new( LiteralExpr::new(Box::new(Str(token_str)), token.line) )
            },
            TokenType::NUMBER => {
                let number = token.literal.clone().unwrap().parse::<f64>().unwrap();
                Box::new( LiteralExpr::new(Box::new(Number(number)), token.line) )
            },
            TokenType::NIL => Box::new( LiteralExpr::new(Box::new(NIL), token.line) ),
            TokenType::TRUE => Box::new( LiteralExpr::new(Box::new(Bool(true)), token.line) ),
            TokenType::FALSE => Box::new( LiteralExpr::new(Box::new(Bool(false)), token.line) ),
            _ => {
                handle_error(&token.line, ErrorType::SyntacticError, 
                    format!("Error at {0}: Expect expression.", token.lexeme).as_str());
                process::exit(SYNTAXIC_ERROR_CODE);
            }
        };
        self.next();
        expr
    }
    
    fn non_binary_expr(&mut self) -> Box<dyn Expression> 
    {
        if self.current_index >= self.size {
            self.exit_error(&self.tokens_list[self.current_index].line, "Error: Expect expression.");
        }
        let token = &self.tokens_list[self.current_index];
        match token.token_type {
            TokenType::MINUS => self.get_unary_expr(token, UnaryOperator::MINUS),
            TokenType::BANG => self.get_unary_expr(token, UnaryOperator::BANG),
            _ => {
                let simple_expr = self.simple_expression();
                return self.assignment_expr(simple_expr);
            }
        }
    }


    fn assignment_expr(&mut self, simple_expr: Box<dyn Expression>) -> Box<dyn Expression> {
        let token = &self.tokens_list[self.current_index - 1];
        if token.token_type == TokenType::RIGHTBRACE || self.current_index + 1 >= self.size {
            return simple_expr
        }
        // if self.current_index + 1 >= self.size {
        //     handle_error(&token.line, ErrorType::SyntacticError, "Unexpected end of file");
        //     process::exit(SYNTAXIC_ERROR_CODE)
        // }

        let ident_str = token.lexeme.to_string();

        let mut next_token = &self.tokens_list[self.current_index];
        if next_token.token_type == TokenType::EQUAL {
            self.next();
            let expr = self.expression();
            return Box::new(IdentifierExpr::new(ident_str, Some(expr), next_token.line));
        }
        else if next_token.token_type != TokenType::DOT {
            return self.callable_expr(simple_expr);
        }

        let mut get_set_expr: Box<dyn Expression> = simple_expr;
        loop {
            self.next();
            let mut get_set_expr_temp = InstanceGetSetExpr::new(get_set_expr, 
                self.simple_expression(), None, next_token.line);
            next_token = &self.tokens_list[self.current_index];
            if next_token.token_type != TokenType::DOT {
                if next_token.token_type == TokenType::EQUAL {
                    self.next();
                    let expr = self.expression();
                    get_set_expr_temp.value_to_assign = Some(expr);
                    return Box::new(get_set_expr_temp);
                }
                else {
                    return self.callable_expr(Box::new(get_set_expr_temp)); 
                }
            }
            get_set_expr = Box::new(get_set_expr_temp);
        }
        
        
    }

    fn callable_expr(&mut self, prev_func_expr: Box<dyn Expression>) -> Box<dyn Expression> {
        if self.current_index > self.size - 2 || self.size == 1 {
            return prev_func_expr;
        }
        let token = self.current_token();
        let line = token.line;
        if token.token_type == TokenType::LEFTPAREN {
            let mut params: Vec<Box<dyn Expression>> = Vec::new();
            self.next();
            if self.current_token().token_type != TokenType::RIGHTPAREN {
                loop {
                    params.push(self.expression());
                    if self.current_token().token_type != TokenType::COMMA {
                        break;
                    } 
                    self.next();
                }
            }
            self.check_token(TokenType::RIGHTPAREN, ")");
               
            let callable = CallExpr::new(prev_func_expr, params, line);
            return self.callable_expr(Box::new(callable));
        }
        else if token.token_type == TokenType::DOT {
            self.next();
            let get_set_expr = InstanceGetSetExpr::new(prev_func_expr, 
                self.simple_expression(), None, line);
            return self.callable_expr(Box::new(get_set_expr));
        }
        prev_func_expr
    }

    fn exit_error(&self, line: &u32, text: &str) {
        handle_error(line, ErrorType::SyntacticError, text);
        process::exit(SYNTAXIC_ERROR_CODE);  
    }

    fn check_token_valid(&self, tokentype: TokenType, lexeme: &str) {
        if self.current_index >= self.size || (&self.tokens_list[self.current_index]).token_type != tokentype  {
            self.exit_error(&self.tokens_list[self.current_index].line, 
                format!("Error: Expected character {}", lexeme).as_str());
        }
    }
    
    pub fn next(&mut self) {
        self.current_index += 1;
    }

    fn get_unary_expr(&mut self, token: &Token, op: UnaryOperator) -> Box<dyn Expression> {
        self.next();
        let child_expr = self.non_binary_expr();      
        let expr = UnaryExpr::new(op, child_expr, token.line);
        Box::new(expr)
    }

    pub fn expression(&mut self) -> Box<dyn Expression> {
        let start_expr = self.non_binary_expr();
        let op_prior_list = self.op_priority_list.clone();
        let expr = self.get_expr_op_priority(start_expr, &op_prior_list);
        expr
    }

    pub fn current_token(&self) -> &Token {
        &self.tokens_list[self.current_index]
    }

    pub fn check_token(&mut self, tokentype: TokenType, lexeme: &str) {
    let token = self.current_token();
    if token.token_type != tokentype {
        handle_error(&token.line, ErrorType::SyntacticError, 
            format!("Error at '{}': Expect {}", token.lexeme, lexeme).as_str());
        process::exit(SYNTAXIC_ERROR_CODE);  
    }  
    self.next();
}
    
}
