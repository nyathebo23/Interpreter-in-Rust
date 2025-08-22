use std::borrow::Cow;
use std::process;
use std::rc::Rc;

use crate::error_handler::SYNTAXIC_ERROR_CODE;
use crate::interpreter::Interpreter;
use crate::parser::block_scopes::BlockScopes;
use crate::parser::declarations::{Number, Object, ValueObjTrait, NIL};
use crate::parser::expressions::{Expression};
use crate::scanner::declarations::TokenType;
use crate::statements::function_stmt::block_func_statement;
use crate::statements::{BlockFuncStatement, Statement};
use crate::parser::declarations::Type;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct Function {
    pub name: Rc<String>,
    pub params_names: Rc<Vec<String>>,
    pub statement: Rc<BlockFuncStatement>,
}

impl Object for Function  {

    fn get_type(&self) -> Type {
        Type::FUNCTION
    }

    fn dyn_clone(&self) -> Box<dyn Object> {
        Box::new(
            Function {
                name: self.name.clone(),
                params_names: self.params_names.clone(),
                statement: self.statement.clone()
            }
        )
    }

    fn to_str(&self) -> std::borrow::Cow<'static, str> {
        Cow::Owned(format!("<fn {}>", self.name))
    }
}

impl ValueObjTrait for Function {
    fn as_function(&self) -> Option<&Function> {
        Some(self)
    }
}


impl ToString for Function {
    fn to_string(&self) -> String {
        self.to_str().to_string()
    }
}

impl Function {

    pub fn call(&self, params: &Vec<Box<dyn Expression>>, out_func_state: &mut BlockScopes) -> Box<dyn Object> {
        if self.params_names.len() != params.len() {
            process::exit(SYNTAXIC_ERROR_CODE);
        }
        if self.name.as_str() == "clock" {
            return Box::new(Number(clock() as f64));
        }
        out_func_state.start_child_block();
        for (param_name, param_val) in self.params_names.iter().zip(params.iter()) {
            let param_value = param_val.evaluate(out_func_state);
            out_func_state.set_init_variable(param_name, param_value);
        }
        self.statement.run(out_func_state);
        let ret_value = match out_func_state.get_variable(&String::from("return")) {
            Some(ret_value ) => ret_value,
            None => Box::new(NIL)
        };
        println!("{} {}", ret_value.to_string(), self.params_names.clone()[0]);
        out_func_state.end_child_block();
        ret_value
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
        let ident_str = identifier.lexeme.to_string();
        let function = Function {
            name: ident_str.into(),
            params_names: params.into(),
            statement: Rc::new(statement),
        };
        interpreter.state.define_function(&function.name.clone(), function);
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
        statement: Rc::new(BlockFuncStatement {
            statements: Vec::new()
        })
    }
}