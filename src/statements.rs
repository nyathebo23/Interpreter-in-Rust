use std::process;

use crate::{parser::expression, scanner::declarations::{Token, TokenType}};

mod declarations;


pub fn print_statement(tokens_list: &Vec<Token>, mut index: &mut usize, size_list: usize) {
    let expr = expression(&tokens_list, &mut index, size_list);
    if tokens_list[*index].token_type != TokenType::SEMICOLON {
        process::exit(65);  
    }  
    let res = expr.evaluate();
    println!("{}", res.to_str());
}