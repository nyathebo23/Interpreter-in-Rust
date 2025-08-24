

use std::usize::MAX;

use crate::{function_manage::Function, parser::{block_scopes::BlockScopes, declarations::Object, expressions::Expression}};
mod simple_statement;
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
        let condition = get_condition(self.condition.evaluate(state));
        if condition {
            *current_stmt_ind += 1;
        }
        else {
            *current_stmt_ind += self.steps;

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
                hashmap.insert(return_key.clone(), value.dyn_clone());
            }
            else {
                ind += 1;
            }
        } 
        for _ in 0..ind {
            state.end_child_block();
            println!("{} ", ind);
        }
        *current_stmt_ind = MAX;
    }
}

impl ReturnStatement  {
    pub fn new(expr: Box<dyn Expression>) -> ReturnStatement {
        ReturnStatement { expression: expr }
    }
}


pub struct FunctionDeclStatement {
    function_decl: Function,
}

impl Statement for FunctionDeclStatement {
    fn run(&self, state: &mut BlockScopes, current_stmt_ind: &mut usize) {
        state.define_function(&self.function_decl.name.clone(), self.function_decl.clone());
        *current_stmt_ind += 1;
    }
}

fn get_condition(cond_option: Box<dyn Object>) -> bool {
    match cond_option.as_bool() {
        Some(cond) => cond.0,
        None => false
    }
}