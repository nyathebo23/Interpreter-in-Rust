use crate::parser::{block_scopes::BlockScopes, expressions::Expression};


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
        let condition_option = self.condition.evaluate(state);
        let condition =  match condition_option.as_bool() {
            Some(cond) => cond.0,
            None => false
        };
        if condition {
            self.body.run(state);
        }
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
        let condition_option = self.condition.evaluate(state);
        let condition =  match condition_option.as_bool() {
            Some(cond) => cond.0,
            None => false
        };
        if condition {
            self.body.run(state);
        }
        else {
            
            for stmt in self.else_if_options.iter() {
                stmt.run(state);
            }

            if let Some(else_stmt) = &self.else_statement {
                else_stmt.run(state);
            }
        }
    }
}