
use crate::function::clock_declaration;
use crate::interpreter::block_scopes::BlockScopes;
use crate::parser::Parser;
use crate::statements::controlflow_stmts::statement;
use crate::statements::Statement;
pub mod block_scopes;
pub mod expr_impl;
mod utils;

pub struct Interpreter<'a> {
    pub parser: Parser<'a>,
    pub state: BlockScopes
}

impl Interpreter<'_> {
    
    pub fn new(parser: Parser<'_>) -> Interpreter {
        Interpreter { parser, state: BlockScopes::new() }
    }

    pub fn compile(&mut self) {
        self.state.define_function(&String::from("clock"), clock_declaration());
        let mut stmts: Vec<Box<dyn Statement>> = Vec::new();
        while self.parser.current_index < self.parser.size {
            stmts.append(&mut statement(self));
        }
        Self::run(&mut self.state, &mut stmts);
    }

    pub fn run(state: &mut BlockScopes, stmts: &Vec<Box<dyn Statement>>) {
        let mut index = 0;
        while index < stmts.len() {
            let statement: &Box<dyn Statement>  = &stmts[index];
            statement.run(state, &mut index);
        }
    }

}