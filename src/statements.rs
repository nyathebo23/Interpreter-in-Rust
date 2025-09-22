

use std::{cell::RefCell, collections::HashMap, rc::Rc, usize::MAX};

use crate::class::Class;
use crate::error_handler::{handle_error, ErrorType};
use crate::interpreter::block_scopes::BlockScopes;
use crate::parser::declarations::{RefObject, Type};
use crate::function::Function;
use crate::parser::expressions::Identifier;
use crate::parser::{declarations::Object, expressions::Expression};
use crate::scanner::declarations::Token;
mod simple_statement;
pub mod classes_decl_stmt;
pub mod controlflow_stmts;
pub mod function_stmt;


pub trait Statement {
    fn run(&self, state: &mut BlockScopes, current_stmt_ind: &mut usize);
}

pub struct PrintStatement {
    pub expression: Box<dyn Expression>
}

impl Statement for PrintStatement  {
    fn run(&self, state: &mut BlockScopes, current_stmt_ind: &mut usize) {
       let value = &self.expression.evaluate(state);
       println!("{}", value.to_str()); 
       *current_stmt_ind += 1;
    }
}

pub struct VarStatement {
    pub name: String,
    pub expression: Box<dyn Expression>
}

impl Statement for VarStatement {
    fn run(&self, state: &mut BlockScopes, current_stmt_ind: &mut usize) {
        let expr_value = self.expression.evaluate(state);
        state.set_init_variable(&self.name, expr_value);
       *current_stmt_ind += 1;
    }
}

pub struct ExprStatement {
    pub expression: Box<dyn Expression>
}

impl Statement for ExprStatement {
    fn run(&self, state: &mut BlockScopes, current_stmt_ind: &mut usize) {
        self.expression.evaluate(state);
       *current_stmt_ind += 1;
    }
}


pub struct JumpStatement {
    pub condition: Box<dyn Expression>,
    pub steps: usize
}


impl Statement for JumpStatement {
    fn run(&self, state: &mut BlockScopes, current_stmt_ind: &mut usize) {
        let condition = JumpStatement::get_condition(self.condition.evaluate(state));
        if condition {
            *current_stmt_ind += 1;
        }
        else {
            *current_stmt_ind += self.steps;

        }
    }
}

impl JumpStatement {
    fn get_condition(cond_option: Box<dyn Object>) -> bool {
        match cond_option.as_bool() {
            Some(cond) => cond.0,
            None => false
        }
    }
}

pub struct BackToStatement {
    pub steps: usize
}

impl Statement for BackToStatement {
    fn run(&self, _state: &mut BlockScopes, current_stmt_ind: &mut usize) {
        *current_stmt_ind -= self.steps;
    }
}

pub struct GoToStatement {
    pub steps: usize
}

impl Statement for GoToStatement {
    fn run(&self, _state: &mut BlockScopes, current_stmt_ind: &mut usize) {
        *current_stmt_ind += self.steps;
    }
}

pub struct StartBlockStatement {
    
}

impl Statement for StartBlockStatement {
    fn run(&self, state: &mut BlockScopes, current_stmt_ind: &mut usize) {
        state.start_child_block();
        *current_stmt_ind += 1;
    }
}

pub struct EndBlockStatement {
    
}

impl Statement for EndBlockStatement {
    fn run(&self, state: &mut BlockScopes, current_stmt_ind: &mut usize) {
        state.end_child_block();
        *current_stmt_ind += 1;
    }
}


pub struct ReturnStatement {
    pub expression: Box<dyn Expression>,
}

impl Statement for ReturnStatement {
    fn run(&self, state: &mut BlockScopes, current_stmt_ind: &mut usize) {
        let value = self.expression.evaluate(state);
        let return_key = String::from("return");
        let mut ind = 0;
        for hashmap in state.vars_nodes_map.iter_mut().rev() {
            if let Some(_val) = hashmap.get(&return_key) {
                hashmap.insert(return_key.clone(), Rc::new(RefCell::new(value.dyn_clone())));
                break;
            }
            else {
                ind += 1;
            }
        } 
        for _ in 0..ind {
            state.end_child_block();
        }
        *current_stmt_ind = MAX;
    }
}

impl ReturnStatement  {
    pub fn new(expr: Box<dyn Expression>) -> ReturnStatement {
        ReturnStatement { expression: expr }
    }
}

#[derive(Clone)]
pub struct FunctionDeclStatement {
    pub function_decl: Function,
    pub extern_variables: Vec<Identifier>
}


impl Statement for FunctionDeclStatement {
    fn run(&self, state: &mut BlockScopes, current_stmt_ind: &mut usize) {
        let func_copy = Function {
            name: self.function_decl.name.clone(),
            params_names: self.function_decl.params_names.clone(),
            statements: self.function_decl.statements.clone(),
            extra_map: self.get_outfunc_variables(&state)
        };
        state.define_function(&self.function_decl.name.clone(), func_copy.clone());
        *current_stmt_ind += 1;
    }
}

impl FunctionDeclStatement {
    fn get_outfunc_variables(&self, state: &BlockScopes) -> HashMap<String, RefObject> {
        let mut result_map: HashMap<String, RefObject>  = HashMap::new();

        for identifier in &self.extern_variables {
            for hashmap in state.vars_nodes_map.iter().rev() {
                let val = hashmap.get(&identifier.value);
                if let Some(value) = val {
                    result_map.insert(identifier.value.to_string(), value.clone());
                    break;
                }
            }
            handle_error(&identifier.line, ErrorType::RuntimeError, 
            format!("Undefined variable '{}'.", identifier.value.as_str()).as_str());
        }
        result_map
    }
}

pub struct ClassDeclStatement {
    super_class_token: Option<Token>,
    class: Class
}

impl Statement for ClassDeclStatement {
    fn run(&self, state: &mut BlockScopes, current_stmt_ind: &mut usize) {
        if let Some(supclass_token) = &self.super_class_token {
            let super_class_name = supclass_token.lexeme.to_string();
            if let Some(super_class_obj) = state.get_variable(&super_class_name) {
                if super_class_obj.get_type() == Type::CLASS {
                    let mut class = self.class.clone();
                    let super_class = super_class_obj.as_class().unwrap();
                    for (funcname, func)  in &super_class.methods {
                        if !class.methods.contains_key(funcname) {
                            //class.methods.insert(funcname.to_string(), func.clone());
                            class.inherited_methods.insert(funcname.to_string(), func.clone());
                        }
                    }
                    if let None = class.constructor {
                        if let Some(initmethod) = &super_class.constructor {
                            class.constructor = Some(initmethod.clone());
                            class.inherited_methods.insert(initmethod.function_decl.name.to_string(), initmethod.clone());
                        }
                    }
                    class.super_class = Some(Box::new(super_class.clone()));
                    state.define_class(&class.name, class.clone());
                    *current_stmt_ind += 1;
                    return;
                }
            }
            handle_error(&supclass_token.line, ErrorType::RuntimeError, "Superclass must be a class.");
        }
        state.define_class(&self.class.name, self.class.clone());
        *current_stmt_ind += 1;
    }
}