
use crate::error_handler::*;
use crate::parser::declarations::*;
use crate::parser::operators_decl::*;
use crate::parser::utils::check_equality;
use crate::parser::utils::perform_add;
use crate::parser::utils::perform_comparison;
use crate::parser::utils::perform_num_op;
use std::process;

pub trait Expression {
    fn evaluate(&self) -> Box<dyn Object>;
    fn to_string(&self) -> String;
}

pub struct BinaryExpr {
    pub operator: BinaryOperator,
    pub value1: Box<dyn Expression>,
    pub value2: Box<dyn Expression>,
    pub line: u32
}

pub struct UnaryExpr {
    pub operator: UnaryOperator,
    pub value: Box<dyn Expression>,
    pub line: u32
}

pub struct LiteralExpr {
    pub value: Box<dyn Object>,
}

pub struct GroupExpr  {
    pub value: Box<dyn Expression>,
}

impl Expression for LiteralExpr {
    fn evaluate(&self) -> Box <dyn Object> {
        return self.value.dyn_clone();
    }

    fn to_string(&self) -> String {
        self.value.to_str().to_string()
    }
}

impl Expression for GroupExpr {
    fn evaluate(&self) -> Box <dyn Object> {
        return self.value.evaluate();
    }

    fn to_string(&self) -> String {
        let child = self.value.to_string();
        format!("(group {child})")
    }
}


impl  Expression for UnaryExpr {
    fn evaluate(&self) -> Box <dyn Object> {
        let value_evaluated = self.value.evaluate();
        match self.operator {
            UnaryOperator::BANG => {
                match value_evaluated.get_type() {
                    Type::BOOLEAN => {
                        let bool = value_evaluated.as_bool().unwrap();
                        return Box::new(Bool(!bool.0));
                    },
                    Type::NIL => {
                        return Box::new(Bool(true));
                    },
                    _ => {
                        return Box::new(Bool(false));
                    }
                }
            },
            UnaryOperator::MINUS => {
                match value_evaluated.get_type() {
                    Type::NUMBER => {
                        let num = value_evaluated.as_number().unwrap();
                        return Box::new(Number(-num.0));
                    },
                    _ => {
                        handle_error(&self.line, ErrorType::RuntimeError, "Operand must be a number.");
                        process::exit(70);
                    }
                }
            }
        }
    }

    fn to_string(&self) -> String {
        let child = self.value.to_string();
        let op = match self.operator {
            UnaryOperator::BANG => "!",
            UnaryOperator::MINUS => "-"
        };
        format!("({op} {child})")
    }
}

impl  Expression for BinaryExpr {
    
    fn evaluate(&self) -> Box<dyn Object> {

        let val1 = self.value1.evaluate();
        let val2 = self.value2.evaluate();
        match self.operator {
            BinaryOperator::PLUS => {
                perform_add(val1, val2, &self.line)
            },
            BinaryOperator::MINUS => {
                perform_num_op(val1, val2, |x, y| x - y, &self.line)
            },
            BinaryOperator::STAR => {
                perform_num_op(val1, val2, |x, y| x * y, &self.line)
            },
            BinaryOperator::SLASH => {
                perform_num_op(val1, val2, |x, y| x / y, &self.line)
            },
            BinaryOperator::EQUALEQUAL => {
                check_equality(val1, val2, true)
            },
            BinaryOperator::BANGEQUAL => {
                check_equality(val1, val2, false)
            },
            BinaryOperator::GREATER => {
                perform_comparison(val1, val2, |x, y| x > y, &self.line)
            },
            BinaryOperator::GREATEREQUAL => {
                perform_comparison(val1, val2, |x, y| x >= y, &self.line)                
            },
            BinaryOperator::LESS => {
                perform_comparison(val1, val2, |x, y| x < y, &self.line)                
            },
            BinaryOperator::LESSEQUAL => {
                perform_comparison(val1, val2, |x, y| x <= y, &self.line)                
            },
        }
    }

    fn to_string(&self) -> String {
        let child1 = self.value1.to_string();
        let child2 = self.value2.to_string();
        let operator_map = binary_op_map();
        format!("({} {child1} {child2})", operator_map[&self.operator]) 
    }
}