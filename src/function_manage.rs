use std::collections::HashMap;
use std::process;
use std::rc::Rc;

use crate::error_handler::SYNTAXIC_ERROR_CODE;
use crate::interpreter::Interpreter;
use crate::parser::block_scopes::BlockScopes;
use crate::parser::declarations::{Number, Object, NIL};
use crate::parser::expressions::{Expression};
use crate::scanner::declarations::TokenType;
use crate::statements::function_stmt::block_func_statement;
use crate::statements::{BlockFuncStatement, Statement};

use std::time::{SystemTime, UNIX_EPOCH};


pub struct Function {
    pub name: Rc<String>,
    pub params_names: Rc<Vec<String>>,
    pub state: BlockScopes,
    pub statement: Rc<BlockFuncStatement>,
}

impl Function {

    pub fn call(&mut self, params: &Vec<Box<dyn Expression>>, out_func_state: &mut BlockScopes) -> Box<dyn Object> {
        if self.params_names.len() != params.len() {
            process::exit(SYNTAXIC_ERROR_CODE);
        }
        if self.name.as_str() == "clock" {
            return Box::new(Number(clock() as f64));
        }
        for (param_name, param_val) in self.params_names.iter().zip(params.iter()) {
            self.state.set_init_variable(param_name, param_val.evaluate(out_func_state));
        }
        self.statement.run(&mut self.state);
        match self.state.get_global_variable(&String::from("return")) {
            Some(ret_value ) => ret_value,
            None => Box::new(NIL)
        }
    }

    pub fn duplicate_func_scope(&self) -> Function {
        let mut functions_decls = HashMap::new();
        for (key, val) in self.state.func_declarations.iter() {
            functions_decls.insert(key.clone(), val.duplicate_func_scope());
        }
        let state = BlockScopes::new(functions_decls);
        Function { 
            name: self.name.clone(),
            params_names: self.params_names.clone(), 
            state: state, 
            statement: self.statement.clone(), 
        }
    }
}


pub fn fun_declaration(interpreter: &mut Interpreter) {
    let token = interpreter.parser.current_token();
    if token.token_type == TokenType::FUN {
        interpreter.next();
        let identifier = interpreter.parser.current_token().clone();
        interpreter.next();        
        let mut params: Vec<String> = Vec::new();
        interpreter.check_token(TokenType::LEFTPAREN, "(");
        let mut current_token = interpreter.parser.current_token();
        if current_token.token_type != TokenType::RIGHTPAREN {
            loop {
                current_token = interpreter.parser.current_token();
                params.push(current_token.lexeme.to_string());
                interpreter.check_token(TokenType::IDENTIFIER, "Identifier");
                if interpreter.parser.current_token().token_type != TokenType::COMMA {
                    break;
                } 
                interpreter.next();
            }
        }
        interpreter.check_token(TokenType::RIGHTPAREN, ")");
        interpreter.check_token(TokenType::LEFTBRACE, "{");
        let statement = block_func_statement(interpreter);
        let mut functions_decls = HashMap::new();
        for (func_name, function) in interpreter.state.func_declarations.iter() {
            functions_decls.insert(func_name.clone(), function.duplicate_func_scope());
        }
        let function = Function {
            name: identifier.lexeme.to_string().into(),
            params_names: params.into(),
            statement: Rc::new(statement),
            state: BlockScopes::new(functions_decls),
        };
        interpreter.state.define_function(identifier.lexeme.to_string(), function);
    }
}

fn clock() -> u64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

pub fn clock_declaration() -> Function {
    // let return_stmt: Box<dyn Statement> = Box::new(ReturnStatement {
    //     expression: Box::new(LiteralExpr { value: Box::new(Number(clock() as f64)) } )
    // });
    // let stmts: Vec<Box<dyn Statement>> = Vec::from([return_stmt]);
    Function { 
        name: "clock".to_string().into(), 
        params_names: Vec::new().into(), 
        state: BlockScopes::new(HashMap::new()), 
        statement: Rc::new(BlockFuncStatement {
            statements: Vec::new()
        })
    }
}