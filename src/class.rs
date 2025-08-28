use std::{borrow::Cow, cell::RefCell, collections::HashMap, process, rc::Rc};

use crate::error_handler::{handle_error, ErrorType};
use crate::function::Function;
use crate::interpreter::block_scopes::BlockScopes;
use crate::{error_handler::RUNTIME_ERROR_CODE};
use crate::parser::expressions::{Expression, InstanceGetSetExpr};
use crate::parser::declarations::{Object, RefObject, Type, ValueObjTrait};



#[derive(Clone)]
pub struct Class {
    pub name: String,
    pub methods: Vec<Function>,
    pub constructor: Option<Function>
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
            name: self.name.clone(),
            methods: self.methods.clone(),
            constructor: self.constructor.clone()
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
        if let Some(val) = attributes_mut.get(field) {
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
    pub fn call(&self, params: &Vec<Box<dyn Expression>>, out_func_state: &mut BlockScopes, line: &u32) -> ClassInstance {
        let instance = ClassInstance {
            class: Rc::new(self.clone()),
            attributes: Rc::new(RefCell::new(HashMap::new())) 
        };
        let this = String::from("this");
        let mut attributes_mut = instance.attributes.borrow_mut();
        if let Some(init_method) = &self.constructor {
            let name = init_method.name.to_string();
            let instance_copy: Box<dyn Object> = Box::new(instance.clone());
            let mut func_copy = init_method.clone();
            func_copy.extra_map.insert(this.clone(), Rc::new(RefCell::new(instance_copy)));
            func_copy.call(params, out_func_state, line);
            attributes_mut.insert(name, Rc::new(RefCell::new(Box::new(func_copy))));
        }
        //let mut attrs = HashMap::new();
        for func in self.methods.iter() {
            let name = func.name.to_string();
            let instance_copy: Box<dyn Object> = Box::new(instance.clone());
            let mut func_copy = func.clone();
            func_copy.extra_map.insert(this.clone(), Rc::new(RefCell::new(instance_copy)));
            attributes_mut.insert(name, Rc::new(RefCell::new(Box::new(func_copy))));
            // let func_obj: Box<dyn Object> = Box::new(func_copy);
            // attrs.insert(name, Rc::new(RefCell::new(func_obj)));
        }
        //let instance_clone = instance.clone();
        // let mut attrs_mut = instance.attributes.borrow_mut();
        // *attrs_mut = attrs;
        instance.clone()
    }
}


impl Expression for InstanceGetSetExpr {
    fn evaluate(&self, state_scope: &mut BlockScopes) -> Box<dyn Object> {
        let mut obj = self.instance.evaluate(state_scope);
        if obj.get_type() != Type::CLASSINSTANCE {
            handle_error(&self.line, ErrorType::RuntimeError, 
                "Can only access property on class instance");
            process::exit(RUNTIME_ERROR_CODE);
        }

        let class_instance: &mut ClassInstance = obj.as_class_instance().unwrap();
        let (identifier, prop) = self.property.value_from_class_instance(class_instance, state_scope);
        
        if let Some(value) =  &self.value_to_assign {
            let evaluated_value = value.evaluate(state_scope);
            class_instance.set(&identifier, evaluated_value.dyn_clone());
            return evaluated_value;
        }
        else {
            if let None = prop {
                handle_error(&self.line, ErrorType::RuntimeError, 
                    format!("Undefined property '{}'", identifier).as_str());
                process::exit(RUNTIME_ERROR_CODE);   
            }
            return prop.unwrap();
        } 
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

    fn get_line(&self) -> u32 {
        self.line
    }

    fn to_string(&self) -> String {
        self.instance.to_string()
    }
}
