
use crate::error_handler::*;
use crate::parser::declarations::*;
use std::process;

pub trait Expression {
    fn evaluate(&self) -> BasicType;
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
    pub value: BasicType,
}

pub struct GroupExpr  {
    pub value: Box<dyn Expression>,
}

impl Expression for LiteralExpr {
    fn evaluate(&self) -> BasicType {
        return self.value.clone();
    }

    fn to_string(&self) -> String {
        self.value.to_str()
    }
}

impl Expression for GroupExpr {
    fn evaluate(&self) -> BasicType {
        return self.value.evaluate();
    }

    fn to_string(&self) -> String {
        let child = self.value.to_string();
        format!("(group {child})")
    }
}

fn perform_comparison<F>(data1: BasicType, data2: BasicType, f: F, line: &u32) -> BasicType 
where F: Fn(f64, f64) -> bool
{
    match (data1, data2) {
        (BasicType::NUMBER(num1), BasicType::NUMBER(num2)) => BasicType::BOOLEAN(f(num1, num2)),
        _ => {
            handle_error(line, ErrorType::RuntimeError, "Operand must be a number.");
            process::exit(70);
        }
    }
}

impl  Expression for UnaryExpr {
    fn evaluate(&self) -> BasicType {
        let value_evaluated = self.value.evaluate();
        match self.operator {
            UnaryOperator::BANG => {
                match value_evaluated {
                    BasicType::BOOLEAN(boolean ) => {
                        return BasicType::BOOLEAN(!boolean);
                    },
                    BasicType::NIL => {
                        return BasicType::BOOLEAN(true);
                    },
                    _ => {
                        return BasicType::BOOLEAN(false);
                    }
                }
            },
            UnaryOperator::MINUS => {
                match value_evaluated {
                    BasicType::NUMBER(num) => {
                        return BasicType::NUMBER(-num);
                    },
                    _ => {
                        handle_error(&self.line, ErrorType::RuntimeError, "Operand must be a number.");
                    }
                }
            }
        }
        return self.value.evaluate();
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
    
    fn evaluate(&self) -> BasicType {

        let val1 = self.value1.evaluate();
        let val2 = self.value2.evaluate();
        match self.operator {
            BinaryOperator::PLUS => {
                let result = (val1 + val2).unwrap_or_else(|err| {
                    handle_error(&self.line, ErrorType::RuntimeError, err.as_str());
                    process::exit(70);
                });
                result
            },
            BinaryOperator::MINUS => {
                let result = (val1 - val2).unwrap_or_else(|err| {
                    handle_error(&self.line, ErrorType::RuntimeError, err.as_str());
                    process::exit(70);
                });
                result 
            },
            BinaryOperator::STAR => {
                let result = (val1 * val2).unwrap_or_else(|err| {
                    handle_error(&self.line, ErrorType::RuntimeError, err.as_str());
                    process::exit(70);
                });
                result
            },
            BinaryOperator::SLASH => {
                let result = (val1 / val2).unwrap_or_else(|err| {
                    handle_error(&self.line, ErrorType::RuntimeError, err.as_str());
                    process::exit(70);
                });
                result
            },
            BinaryOperator::EQUALEQUAL => BasicType::BOOLEAN(val1 == val2),
            BinaryOperator::BANGEQUAL => BasicType::BOOLEAN(val1 != val2),
            BinaryOperator::GREATER => perform_comparison(val1, val2, |x, y| x > y, &self.line),
            BinaryOperator::GREATEREQUAL => perform_comparison(val1, val2, |x, y| x >= y, &self.line),
            BinaryOperator::LESS => perform_comparison(val1, val2, |x, y| x < y, &self.line),
            BinaryOperator::LESSEQUAL => perform_comparison(val1, val2, |x, y| x <= y, &self.line),
        }
    }

    fn to_string(&self) -> String {
        let child1 = self.value1.to_string();
        let child2 = self.value2.to_string();
        let operator_map = binary_op_map();

        format!("({} {child1} {child2})", operator_map[&self.operator]) 
    }
}