use crate::compiler::environment::Environment; 
use crate::parser::Parser;
use crate::statements::controlflow_stmts::statement;
use crate::statements::Statement;
pub mod environment;
pub struct Compiler<'a> {
    pub parser: Parser<'a>,
    pub environment: Environment
}

impl Compiler<'_> {

    pub fn advance(&mut self) {
        self.parser.next();
    }

    pub fn not_reach_end(&self) -> bool {
        self.parser.current_index < self.parser.size
    }

    pub fn new(parser: Parser<'_>) -> Compiler<'_> {
        Compiler { parser, environment: Environment::new() }
    }

    pub fn compile(&mut self) -> Vec<Box<dyn Statement>>  {
        let mut stmts: Vec<Box<dyn Statement>> = Vec::new();
         while self.not_reach_end() {
            stmts.append(&mut statement(self));
        }
        stmts
    }
}