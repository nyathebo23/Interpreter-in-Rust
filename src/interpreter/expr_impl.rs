

use crate::class::ClassInstance;
use crate::interpreter::{block_scopes::BlockScopes, utils::*};
use crate::parser::declarations::*;
use crate::parser::expressions::*;
use crate::parser::operators_decl::*;
use crate::error_handler::*;

impl Expression for CallExpr  {

    fn evaluate(&self, state_scope: &mut BlockScopes) -> Box<dyn Object> {
        let callable_val = self.callable.evaluate(state_scope);
        if callable_val.get_type() == Type::FUNCTION {
            let func = callable_val.as_function().unwrap();
            func.call(&self.params, state_scope, &self.line)
        }
        else if callable_val.get_type() == Type::CLASS {
            let class_call = callable_val.as_class().unwrap();
            let instance = class_call.call(&self.params, state_scope, &self.line);
            Box::new(instance)
        }
        else {
            handle_error(&self.line, ErrorType::RuntimeError, 
                "Can only call functions and classes.");
             
        }
        
    }

    fn value_from_class_instance(&self, instance: &ClassInstance, state_scope: &mut BlockScopes) -> (String, Option<Box<dyn Object>>) {
        let (identifier, func_option) = self.callable.value_from_class_instance(instance, state_scope);
        if let Some(func) = func_option {
            if func.get_type() != Type::FUNCTION {
                handle_error(&self.line, ErrorType::RuntimeError, "Expect function");
            }
            return (identifier, Some((func.as_function().unwrap()).call(&self.params, state_scope, &self.line)));
        }
        handle_error(&self.line, ErrorType::RuntimeError, format!("No Callable with name '{}'", identifier).as_str());
    }

    fn contains_identifier(&self, ident: &String) -> bool {
        for param in self.params.iter() {
            if param.contains_identifier(ident) {
                return true;
            }
        }
        self.callable.contains_identifier(ident)
    }

    fn get_line(&self) -> u32 {
        self.line
    }

    fn to_string(&self) -> String {
        self.callable.to_string()
    }
}

impl Expression for IdentifierExpr {
    fn evaluate(&self, state_scope: &mut BlockScopes) -> Box<dyn Object> {
        if let Some(value) = state_scope.get_variable(&self.ident_name) {
            match &self.value_to_assign {
                Some(expr_value) => {
                    let val = expr_value.evaluate(state_scope);
                    state_scope.modif_variable(&self.ident_name, val.dyn_clone());
                    return val;
                },
                None => {
                    return value;
                }
            }
        }
        handle_error(&self.line, ErrorType::RuntimeError, 
            format!("Undefined variable '{}'.", self.ident_name).as_str());
        
    }

    fn value_from_class_instance(&self, instance: &ClassInstance, _state_scope: &mut BlockScopes) -> (String, Option<Box<dyn Object>>) {
        (self.ident_name.clone(), instance.get(&self.ident_name))
    }

    fn contains_identifier(&self, ident: &String) -> bool {
        *ident == self.ident_name
    }

    fn get_line(&self) -> u32 {
        self.line
    }

    fn to_string(&self) -> String {
        self.ident_name.to_string()
    }
}

impl Expression for LiteralExpr {
    fn evaluate(&self, _state_scope: &mut BlockScopes) -> Box <dyn Object> {
        return self.value.dyn_clone();
    }

    fn contains_identifier(&self, _ident: &String) -> bool {
        false
    }

    fn value_from_class_instance(&self, _instance: &ClassInstance, _state_scope: &mut BlockScopes) -> (String, Option<Box<dyn Object>>) {
        handle_error(&self.line, ErrorType::RuntimeError, 
            "Can only access property on class instance");
        
    }

    fn get_line(&self) -> u32 {
        self.line
    }

    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

impl Expression for GroupExpr {
    fn evaluate(&self, state_scope: &mut BlockScopes) -> Box <dyn Object> {
        return self.value.evaluate(state_scope);
    }

    fn contains_identifier(&self, ident: &String) -> bool {
        self.value.contains_identifier(ident)
    }

    fn value_from_class_instance(&self, _instance: &ClassInstance, _state_scope: &mut BlockScopes) -> (String, Option<Box<dyn Object>>) {
        handle_error(&self.line, ErrorType::RuntimeError, 
            "Can only access property on class instance");
        
    }

    fn get_line(&self) -> u32 {
        self.line
    }

    fn to_string(&self) -> String {
        let child = self.value.to_string();
        format!("(group {child})")
    }
}


impl  Expression for UnaryExpr {
    fn evaluate(&self, state_scope: &mut BlockScopes) -> Box <dyn Object> {
        let value_evaluated = self.value.evaluate(state_scope);
        match self.operator {
            UnaryOperator::BANG => {
                match value_evaluated.get_type() {
                    Type::BOOLEAN => {
                        let bool = value_evaluated.as_bool().unwrap();
                        Box::new(Bool(!bool.0))
                    },
                    Type::NIL => Box::new(Bool(true)),
                    _ => Box::new(Bool(false))
                }
            },
            UnaryOperator::MINUS => {
                match value_evaluated.get_type() {
                    Type::NUMBER => {
                        let num = value_evaluated.as_number().unwrap();
                        Box::new(Number(-num.0))
                    },
                    _ => {
                        handle_error(&self.line, ErrorType::RuntimeError, "Operand must be a number.");
                        
                    }
                }
            }
        }
    }

    fn contains_identifier(&self, ident: &String) -> bool {
        self.value.contains_identifier(ident)
    }

    fn value_from_class_instance(&self, _instance: &ClassInstance, _state_scope: &mut BlockScopes) -> (String, Option<Box<dyn Object>>) {
        handle_error(&self.line, ErrorType::RuntimeError, 
            "Can only access property on class instance");
        
    }

    fn get_line(&self) -> u32 {
        self.line
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
    
    fn evaluate(&self, state_scope: &mut BlockScopes) -> Box<dyn Object> {

        let val1 = self.value1.evaluate(state_scope);
        match self.operator {
            BinaryOperator::PLUS => {
                let val2 = self.value2.evaluate(state_scope);
                perform_add(val1, val2, &self.line)
            },
            BinaryOperator::MINUS => {
                let val2 = self.value2.evaluate(state_scope);
                perform_num_op(val1, val2, |x, y| x - y, &self.line)
            },
            BinaryOperator::STAR => {
                let val2 = self.value2.evaluate(state_scope);
                perform_num_op(val1, val2, |x, y| x * y, &self.line)
            },
            BinaryOperator::SLASH => {
                let val2 = self.value2.evaluate(state_scope);
                perform_num_op(val1, val2, |x, y| x / y, &self.line)
            },
            BinaryOperator::EQUALEQUAL => {
                let val2 = self.value2.evaluate(state_scope);
                check_equality(val1, val2, true)
            },
            BinaryOperator::BANGEQUAL => {
                let val2 = self.value2.evaluate(state_scope);
                check_equality(val1, val2, false)
            },
            BinaryOperator::GREATER => {
                let val2 = self.value2.evaluate(state_scope);
                perform_comparison(val1, val2, |x, y| x > y, &self.line)
            },
            BinaryOperator::GREATEREQUAL => {
                let val2 = self.value2.evaluate(state_scope);
                perform_comparison(val1, val2, |x, y| x >= y, &self.line)                
            },
            BinaryOperator::LESS => {
                let val2 = self.value2.evaluate(state_scope);
                perform_comparison(val1, val2, |x, y| x < y, &self.line)                
            },
            BinaryOperator::LESSEQUAL => {
                let val2 = self.value2.evaluate(state_scope);
                perform_comparison(val1, val2, |x, y| x <= y, &self.line)                
            },
            BinaryOperator::OR => {
                if let Some(boolean) = val1.as_bool() {
                    if boolean.0 {
                        return val1;
                    }
                }
                self.value2.evaluate(state_scope)
            },
            BinaryOperator::AND => {
                if let Some(boolean) = val1.as_bool() {
                    if !boolean.0 {
                        return val1;
                    }
                }
                self.value2.evaluate(state_scope)            
            }        
        }
    }

    fn value_from_class_instance(&self, _instance: &ClassInstance, _state_scope: &mut BlockScopes) -> (String, Option<Box<dyn Object>>) {
        handle_error(&self.line, ErrorType::RuntimeError, 
            "Can only access property on class instance");
        
    }

    fn contains_identifier(&self, ident: &String) -> bool {
        self.value1.contains_identifier(ident) || self.value1.contains_identifier(ident)
    }

    fn get_line(&self) -> u32 {
        self.line
    }

    fn to_string(&self) -> String {
        let child1 = self.value1.to_string();
        let child2 = self.value2.to_string();
        let operator_map = binary_op_map();
        format!("({} {child1} {child2})", operator_map[&self.operator]) 
    }
}