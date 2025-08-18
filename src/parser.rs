use std::process;
use crate::error_handler::{handle_error, ErrorType};
use crate::parser::declarations::{BasicType, UnaryOperator, MAP_COMP_TOKEN_OP, MAP_PLUS_MINUS_OP, MAP_SLASH_STAR_OP};
use crate::scanner::declarations::*;
mod declarations;
mod expressions;

use crate::parser::expressions::*;

fn expr_comp_precedence(tokens_list: &Vec<Token>, mut index: &mut usize, 
    prec_expr: Box<dyn Expression>, size_list: usize) -> Box<dyn Expression> 
{
    let mut left_expr = prec_expr;
    left_expr = expr_plus_minus_precedence(&tokens_list, &mut index, left_expr, size_list);
    'outer: while *index < size_list {
        let current_token = &tokens_list[*index];
        for (token_type, token_op) in MAP_COMP_TOKEN_OP {
            if token_type == current_token.token_type {
                *index += 1;
                let right_expr0 = non_binary_expr(&tokens_list, &mut index, size_list);
                let right_expr = expr_plus_minus_precedence(&tokens_list, &mut index, right_expr0, size_list);
                left_expr = Box::new(BinaryExpr {
                    operator: token_op,
                    value1: left_expr,
                    value2: right_expr,
                    line: current_token.line
                });
                break;
            }
            break 'outer;
        }
    }
    left_expr
}

fn expr_plus_minus_precedence(tokens_list: &Vec<Token>, mut index: &mut usize, 
    prec_expr: Box<dyn Expression>, size_list: usize) -> Box<dyn Expression> 
{
    let mut left_expr = prec_expr;
    left_expr = expr_star_slash_precedence(&tokens_list, &mut index, left_expr, size_list);
    'outer: while *index < size_list {
        let current_token = &tokens_list[*index];
        for (token_type, token_op) in MAP_PLUS_MINUS_OP {
            if token_type == current_token.token_type  {
                *index += 1;
                let right_expr0 = non_binary_expr(&tokens_list, &mut index, size_list);
                let right_expr = expr_star_slash_precedence(&tokens_list, &mut index, right_expr0, size_list);
                left_expr = Box::new(BinaryExpr {
                    operator: token_op,
                    value1: left_expr,
                    value2: right_expr,
                    line: current_token.line
                });
                break;
            }
            break 'outer;
        }
    }    
    left_expr
}

fn expr_star_slash_precedence(tokens_list: &Vec<Token>, mut index: &mut usize, 
    prec_expr: Box<dyn Expression>, size_list: usize) -> Box<dyn Expression>
{
    let mut left_expr = prec_expr;
    'outer: while *index < size_list {
        let current_token = &tokens_list[*index];
        println!("{} {}", *index, current_token.lexeme);
        for (token_type, token_op) in MAP_SLASH_STAR_OP {
            if token_type == current_token.token_type  {
                *index += 1;
                let right_expr = non_binary_expr(&tokens_list, &mut index, size_list);
                left_expr = Box::new(BinaryExpr {
                    operator: token_op,
                    value1: left_expr,
                    value2: right_expr,
                    line: current_token.line
                });
                println!("{}", left_expr.to_string());
                break;
            }
            break 'outer;
        }
    }    
    left_expr
}

fn non_binary_expr(tokens_list: &Vec<Token>, mut index: &mut usize, size_list: usize) -> Box<dyn Expression> 
 {
    let token = &tokens_list[*index];

    let expr: Box<dyn Expression>  =  match token.token_type {
        TokenType::LEFTPAREN => {
            *index += 1;
            let child_expr = expression(&tokens_list, &mut index, size_list);  
            let expr = GroupExpr {
                value: child_expr,
            };
            let next_token = &tokens_list[*index];
            if *index >= size_list || next_token.token_type != TokenType::RIGHTPAREN {
                handle_error(&next_token.line, ErrorType::SyntacticError, "Error: Expected character ')'");
                process::exit(65);
            }
            Box::new(expr)
        },
        TokenType::STRING => {
            let token_str = token.literal.clone().unwrap();
            Box::new(LiteralExpr { value: BasicType::STRING(token_str)})
        },
        TokenType::NUMBER => {
            let number = token.literal.clone().unwrap().parse::<f64>().unwrap();
            Box::new(LiteralExpr { value: BasicType::NUMBER(number)})
        },
        TokenType::NIL => Box::new(LiteralExpr { value: BasicType::NIL}),
        TokenType::TRUE => Box::new(LiteralExpr { value: BasicType::BOOLEAN(true)}),
        TokenType::FALSE => Box::new(LiteralExpr { value: BasicType::BOOLEAN(false)}), 
        TokenType::MINUS => {
            *index += 1;
            let child_expr = non_binary_expr(&tokens_list, &mut index, size_list); 
            let expr = UnaryExpr {
                operator: UnaryOperator::MINUS,
                value: child_expr,
                line: token.line
            };   
            return Box::new(expr);
        },
        TokenType::BANG => {
            *index += 1;
            let child_expr = non_binary_expr(&tokens_list, &mut index, size_list);      
            let expr = UnaryExpr {
                operator: UnaryOperator::BANG,
                value: child_expr,
                line: token.line
            };
            return Box::new(expr);
        }
        _ => {
            handle_error(&token.line, ErrorType::SyntacticError, format!("Error at {0}: Expect expression.", token.lexeme).as_str());
            process::exit(65);
        }
    };
    *index += 1;
    expr
}

pub fn expression(tokens_list: &Vec<Token>, mut index: &mut usize, size_list: usize) -> Box<dyn Expression> {
    let start_expr = non_binary_expr(&tokens_list, &mut index, size_list);
    let expr = expr_comp_precedence(&tokens_list, &mut index, start_expr, size_list);
    expr
}