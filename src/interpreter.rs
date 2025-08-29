
use crate::compiler::Compiler;
use crate::function::clock_declaration;
use crate::interpreter::block_scopes::BlockScopes;
use crate::statements::Statement;
pub mod block_scopes;
pub mod expr_impl;
mod utils;

pub struct Interpreter<'a> {
    pub compiler: Compiler<'a>,
    pub state: BlockScopes
}

impl Interpreter<'_> {
    
    pub fn new(compiler: Compiler<'_>) -> Interpreter {
        Interpreter { compiler, state: BlockScopes::new() }
    }

    pub fn exec(&mut self) {
        self.state.define_function(&String::from("clock"), clock_declaration());
        let mut stmts = self.compiler.compile();
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