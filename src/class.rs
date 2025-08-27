use std::{borrow::Cow, cell::RefCell, collections::HashMap, process, rc::Rc};

use crate::error_handler::{handle_error, ErrorType};
use crate::interpreter::block_scopes::BlockScopes;
use crate::{error_handler::RUNTIME_ERROR_CODE};
use crate::parser::expressions::{Expression, InstanceGetSetExpr};
use crate::parser::declarations::{Object, RefObject, Type, ValueObjTrait};


#[derive(Clone)]
pub struct Class {
    pub name: String,
}


#[derive(Clone)]
pub struct ClassInstance {
    pub class: Rc<Class>,
    pub attributes: Rc<RefCell<HashMap<String, RefObject>>>,
}


impl Object for Class  {
    fn to_str(&self) -> Cow<'static, str> {
        return Cow::Owned(self.name.to_string());
    }    

    fn get_type(&self) -> Type {
        Type::CLASS
    }

    fn dyn_clone(&self) -> Box<dyn Object> {
        Box::new(Class{
            name: self.name.clone()
        })
    }
}


impl Object for ClassInstance {
    fn to_str(&self) -> Cow<'static, str> {
        return Cow::Owned(self.class.name.to_string() + " instance");
    }    

    fn get_type(&self) -> Type {
        Type::CLASSINSTANCE
    }

    fn dyn_clone(&self) -> Box<dyn Object> {
        Box::new(ClassInstance{
            class: self.class.clone(),
            attributes: self.attributes.clone(),
        })
    } 
}

impl ClassInstance {
    pub fn get(&self, field: &String) -> Option<Box<dyn Object>> {
        if let Some(val) = self.attributes.borrow().get(field) {
            return Some(val.borrow().dyn_clone());
        }
        None
    }

    pub fn set(&mut self, field: &String, object: Box<dyn Object>) {
        let mut attributes_mut = self.attributes.borrow_mut();
        if let Some(val) = self.attributes.borrow_mut().get(field) {
            let mut val_mut = val.borrow_mut();
            *val_mut = object;
        }
        else {
            attributes_mut.insert(field.clone(), Rc::new(RefCell::new(object)));
        }
    }
}


impl ValueObjTrait for Class {
    fn as_class(&self) -> Option<&Class> {
        Some(self)
    }
}

impl ValueObjTrait for ClassInstance  {
    fn as_class_instance(&mut self) -> Option<&mut ClassInstance> {
        Some(self)
    }
}

impl ToString for Class {
    fn to_string(&self) -> String {
        self.name.to_string()
    }
}

impl ToString for ClassInstance {
    fn to_string(&self) -> String {
        self.to_str().to_string()
    }
}




impl Class {
    pub fn call(&self, _params: &Vec<Box<dyn Expression>>, _out_func_state: &mut BlockScopes, _line: &u32) -> ClassInstance {
        let instance = ClassInstance {
            class: Rc::new(self.clone()),
            attributes: Rc::new(RefCell::new(HashMap::new())),
        };
        instance
    }
}


impl Expression for InstanceGetSetExpr {
    fn evaluate(&self, state_scope: &mut BlockScopes) -> Box<dyn Object> {
        let mut obj = self.instance.evaluate(state_scope);
        if obj.get_type() == Type::CLASSINSTANCE {
            let class_instance: &mut ClassInstance = obj.as_class_instance().unwrap();
            let (identifier, prop) = self.property.value_from_class_instance(class_instance, state_scope);

            if let Some(value) =  &self.value_to_assign {
                let evaluated_value = value.evaluate(state_scope);
                match &prop {
                    Some(property_val) => {
                        if property_val.get_type() != Type::FUNCTION && evaluated_value.get_type() != Type::FUNCTION {
                            class_instance.set(&identifier, evaluated_value);
                        }
                    },
                    None => {
                        if evaluated_value.get_type() != Type::FUNCTION {
                            class_instance.set(&identifier, evaluated_value);
                        } 
                    }
                }
            }
            else {
                if let None = prop {
                    handle_error(&self.line, ErrorType::RuntimeError, format!("No property with name '{}'", identifier).as_str());
                    process::exit(RUNTIME_ERROR_CODE);   
                }             
            } 
            return prop.unwrap();
        }
        handle_error(&self.line, ErrorType::RuntimeError, 
            "Can only access property on class instance");
        process::exit(RUNTIME_ERROR_CODE);
    }

    fn contains_identifier(&self, ident: &String) -> bool {
        match &self.value_to_assign {
            Some(val) => val.contains_identifier(ident),
            None => false
        }
    }

    fn value_from_class_instance(&self, _instance: &ClassInstance, _state_scope: &mut BlockScopes) -> (String, Option<Box<dyn Object>>) {
        process::exit(RUNTIME_ERROR_CODE)
    }

    fn to_string(&self) -> String {
        self.instance.to_string()
    }
}
