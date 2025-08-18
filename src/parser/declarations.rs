
use std::{collections::HashMap, ops::{Add, Div, Mul, Sub}};

use crate::scanner::declarations::*;
use crate::scanner::utils::literal_number;

#[derive(Clone)]
pub enum BasicType {
    STRING(String),
    NUMBER(f64),
    BOOLEAN(bool),
    NIL
}

impl ToString for BasicType {
    fn to_string(&self) -> String {
        match self {
            BasicType::STRING(str) => str.clone(),
            BasicType::NUMBER(num) => literal_number(num.to_string().as_str()),
            BasicType::BOOLEAN(b) => b.to_string(),
            BasicType::NIL => String::from("nil")
        }
    }
}

impl Add for BasicType {
    type Output = Result<BasicType, String> ;
    
    fn add(self, other: BasicType) -> Result<BasicType, String> {
        match (self, other) {
            (BasicType::NUMBER(num1), BasicType::NUMBER(num2)) => Ok(BasicType::NUMBER(num1 + num2)),
            (BasicType::STRING(str1), BasicType::STRING(str2)) => {
                let mut concat_str = str1.clone();
                concat_str.push_str(str2.clone().as_str());
                return Ok(BasicType::STRING(concat_str));
            },
            _ => Err("Operands must be two numbers or two strings.".to_string())
        }
    }
}

impl Mul for BasicType {
    type Output = Result<BasicType, String>;
    
    fn mul(self, other: BasicType) -> Result<BasicType, String> {
        match (self, other) {
            (BasicType::NUMBER(num1), BasicType::NUMBER(num2)) => Ok(BasicType::NUMBER(num1 * num2)),
            _ => Err("Operand must be a number.".to_string())
        }
    }
}

impl Div for BasicType {
    type Output = Result<BasicType, String>;
    
    fn div(self, other: BasicType) -> Result<BasicType, String> {
        match (self, other) {
            (BasicType::NUMBER(num1), BasicType::NUMBER(num2)) => Ok(BasicType::NUMBER(num1 / num2)),
            _ => Err("Operand must be a number.".to_string())
        }
    }
}

impl Sub for BasicType {
    type Output = Result<BasicType, String>;
    
    fn sub(self, other: BasicType) -> Result<BasicType, String> {
        match (self, other) {
            (BasicType::NUMBER(num1), BasicType::NUMBER(num2)) => Ok(BasicType::NUMBER(num1 - num2)),
            _ => Err("Operand must be a number.".to_string())
        }
    }
}


impl PartialEq for BasicType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (BasicType::BOOLEAN(bool1), BasicType::BOOLEAN(bool2)) => *bool1 == *bool2,
            (BasicType::BOOLEAN(bool), BasicType::NIL) => *bool == false,
            (BasicType::NUMBER(num1), BasicType::NUMBER(num2)) => *num1 == *num2,
            (BasicType::STRING(str1), BasicType::STRING(str2)) => *str1 == *str2,
            _ => false
        }
    }
}

#[derive(Hash, PartialEq, Eq)]
pub enum BinaryOperator {
    PLUS,
    MINUS,
    STAR,
    SLASH,
    BANGEQUAL,
    EQUALEQUAL,
    LESS,
    LESSEQUAL,
    GREATER,
    GREATEREQUAL,
}

pub enum UnaryOperator {
    BANG,
    MINUS
}


pub const MAP_COMP_TOKEN_OP: [(TokenType, BinaryOperator); 6] = [
    (TokenType::EQUALEQUAL, BinaryOperator::EQUALEQUAL),
    (TokenType::BANGEQUAL, BinaryOperator::BANGEQUAL),
    (TokenType::LESS, BinaryOperator::LESS),
    (TokenType::LESSEQUAL, BinaryOperator::LESSEQUAL),
    (TokenType::GREATER, BinaryOperator::GREATER),
    (TokenType::GREATEREQUAL, BinaryOperator::GREATEREQUAL),
];

pub const MAP_SLASH_STAR_OP:  [(TokenType, BinaryOperator); 2] = [
    (TokenType::SLASH, BinaryOperator::SLASH),
    (TokenType::STAR, BinaryOperator::STAR),
];

pub const MAP_PLUS_MINUS_OP:  [(TokenType, BinaryOperator); 2] = [
    (TokenType::PLUS, BinaryOperator::PLUS),
    (TokenType::MINUS, BinaryOperator::MINUS),
];

pub fn binary_op_map() -> HashMap<BinaryOperator, &'static str> {
    HashMap::from([
        (BinaryOperator::EQUALEQUAL, "=="),
        (BinaryOperator::BANGEQUAL, "!="),
        (BinaryOperator::LESS, "<"),
        (BinaryOperator::LESSEQUAL, "<="),
        (BinaryOperator::GREATER, ">"),
        (BinaryOperator::GREATEREQUAL, ">="),
        (BinaryOperator::PLUS, "+"),
        (BinaryOperator::MINUS, "-"),
        (BinaryOperator::SLASH, "/"),
        (BinaryOperator::STAR, "*"),
    ])
}