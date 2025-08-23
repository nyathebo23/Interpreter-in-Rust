use std::borrow::Cow;
use std::process;
use std::rc::Rc;

use crate::error_handler::{handle_error, ErrorType, SYNTAXIC_ERROR_CODE};
use crate::parser::block_scopes::BlockScopes;
use crate::parser::declarations::{Number, Object, ValueObjTrait, NIL};
use crate::parser::expressions::{Expression};
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

    pub fn call(&self, params: &Vec<Box<dyn Expression>>, out_func_state: &mut BlockScopes, line: &u32) -> Box<dyn Object> {
        let (expect_params_len, recv_params_len) = (self.params_names.len(), params.len());
        if self.params_names.len() != params.len() {
            handle_error(line, ErrorType::RuntimeError, 
                format!("Expected {} arguments but got {}", expect_params_len, recv_params_len).as_str());
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
        out_func_state.end_child_block();
        ret_value
    }

}




fn clock() -> u64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

pub fn clock_declaration() -> Function {
    Function { 
        name: "clock".to_string().into(), 
        params_names: Vec::new().into(), 
        statement: Rc::new(BlockFuncStatement {
            statements: Vec::new()
        })
    }
}