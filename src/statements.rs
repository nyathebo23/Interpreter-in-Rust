use crate::parser::{block_scopes::BlockScopes, declarations::Object, expressions::Expression};
mod simple_statement;
pub mod controlflow_stmts;
pub mod function_stmt;
pub trait Statement {
    fn run(&self, state: &mut BlockScopes);
}

pub struct PrintStatement {
    pub expression: Box<dyn Expression>
}

impl Statement for PrintStatement  {
    fn run(&self, state: &mut BlockScopes) {
       
       let value = &self.expression.evaluate(state);
       println!("{}", value.to_str()); 
    }
}

pub struct VarStatement {
    pub name: String,
    pub expression: Box<dyn Expression>
}

impl Statement for VarStatement {
    fn run(&self, state: &mut BlockScopes) {
        let expr_value = self.expression.evaluate(state);
        state.set_init_variable(&self.name, expr_value);
    }
}

pub struct ExprStatement {
    pub expression: Box<dyn Expression>
}

impl Statement for ExprStatement {
    fn run(&self, state: &mut BlockScopes) {
        self.expression.evaluate(state);
    }
}

pub struct BlockStatement {
    pub statements: Vec<Box<dyn Statement>>
}

impl Statement for BlockStatement  {
    fn run(&self, state: &mut BlockScopes) {
        state.start_child_block();
        for stmt in self.statements.iter() {
            stmt.run(state);
        }
        state.end_child_block();
    }
}

pub struct PartIfStatement {
    pub condition: Box<dyn Expression>,
    pub body: Box<dyn Statement>,
}

impl Statement for PartIfStatement {
    fn run(&self, state: &mut BlockScopes) {
        self.body.run(state);
    }
}

fn get_condition(cond_option: Box<dyn Object>) -> bool {
    match cond_option.as_bool() {
        Some(cond) => cond.0,
        None => false
    }
}

pub struct IfStatement {
    pub condition: Box<dyn Expression>,
    pub body: Box<dyn Statement>,
    pub else_if_options: Vec<PartIfStatement>,
    pub else_statement: Option<Box<dyn Statement>>
}

impl Statement for IfStatement  {
    fn run(&self, state: &mut BlockScopes) {
    
        let condition = get_condition(self.condition.evaluate(state));
        if condition {
            self.body.run(state);
            if let Some(_return_val) = state.get_variable(&"return".to_string()) {
                return;
            }
        }
        else {
            
            for stmt in self.else_if_options.iter() {
                let condition = get_condition(stmt.condition.evaluate(state));
                if condition {
                    stmt.run(state);
                    if let Some(_return_val) = state.get_variable(&"return".to_string()) {
                        return;
                    }
                    return;
                }               
            }

            if let Some(else_stmt) = &self.else_statement {
                else_stmt.run(state);
                if let Some(_return_val) = state.get_variable(&"return".to_string()) {
                    return;
                }
            }
        }
    }
}

pub struct WhileStatement {
    pub condition: Box<dyn Expression>,
    pub body: Box<dyn Statement>, 
}

impl Statement for WhileStatement  {
    fn run(&self, state: &mut BlockScopes) {
        let mut condition = get_condition(self.condition.evaluate(state));
        while condition {
            self.body.run(state);
            if let Some(_return_val) = state.get_variable(&"return".to_string()) {
                return;
            }
            condition = get_condition(self.condition.evaluate(state));
        }        
    }
}


pub struct ForStatement {
    pub init_declaration: Option<VarStatement>,
    pub init_assignation: Option<ExprStatement>,
    pub condition: Box<dyn Expression>,
    pub body: Box<dyn Statement>, 
    pub last_instruction: Option<Box<dyn Expression>>
}

impl Statement for ForStatement  {
    fn run(&self, state: &mut BlockScopes) {
        if let Some(init_decl) = &self.init_declaration {
            init_decl.run(state);
        }
        else if let Some(init_assign) = &self.init_assignation {
            init_assign.run(state);
        }
        let mut for_condition = get_condition(self.condition.evaluate(state));
        while for_condition {
            self.body.run(state);
            if let Some(_return_val) = state.get_variable(&"return".to_string()) {
                return;
            }
            if let Some(last_instruction) = &self.last_instruction {
                last_instruction.evaluate(state);
            }
            for_condition = get_condition(self.condition.evaluate(state));
        }        
    }
}


pub struct ReturnStatement {
    pub expression: Box<dyn Expression>
}

impl Statement for ReturnStatement {
    fn run(&self, state: &mut BlockScopes) {
        let value = self.expression.evaluate(state);
        let return_key = String::from("return");
        state.set_init_variable(&return_key, value);
    }
}


pub struct BlockFuncStatement {
    pub statements: Vec<Box<dyn Statement>>
}

impl Statement for BlockFuncStatement  {

    fn run(&self, state: &mut BlockScopes) {
        for stmt in self.statements.iter() {
            stmt.run(state);
            if let Some(_return_val) = state.get_variable(&"return".to_string()) {
                break;
            }
        }
    }

}