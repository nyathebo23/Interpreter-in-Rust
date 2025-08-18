use std::collections::HashMap;
use crate::scanner::declarations::*;

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