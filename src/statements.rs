use std::process;

use crate::{parser::expression, scanner::declarations::{Token, TokenType}};

mod declarations;


fn print_statement(tokens_list: &Vec<Token>, mut index: &mut usize, size_list: usize) {
    let expr = expression(&tokens_list, &mut index, size_list);
    if tokens_list[*index].token_type != TokenType::SEMICOLON {
        process::exit(65);  
    }  
    *index += 1;
    let res = expr.evaluate();
    println!("{}", res.to_str());
}

pub fn run(tokens_list: &Vec<Token>, mut index: &mut usize, size_list: usize) {
    while *index < size_list {
        match tokens_list[*index].token_type {
            TokenType::PRINT => {
                *index += 1;
                print_statement(tokens_list, &mut index, size_list);
            },
            _ => {
                let _expr = expression(&tokens_list, &mut index, size_list);
                if tokens_list[*index].token_type != TokenType::SEMICOLON {
                    process::exit(65);  
                }  
                *index += 1;
            } 
        } 
    }

}