use std::{borrow::Cow, cell::RefCell, collections::HashMap, process, rc::Rc};

use crate::error_handler::{handle_error, ErrorType};
use crate::interpreter::block_scopes::BlockScopes;
use crate::statements::FunctionDeclStatement;
use crate::{error_handler::RUNTIME_ERROR_CODE};
use crate::parser::expressions::{Expression, InstanceGetSetExpr};
use crate::parser::declarations::{Object, RefObject, Type, ValueObjTrait};

#[derive(Clone)]
pub struct Class {
    pub name: String,
    pub methods: HashMap<String, FunctionDeclStatement>,
    pub inherited_methods: HashMap<String, FunctionDeclStatement>,
    pub constructor: Option<FunctionDeclStatement>,
    pub super_class: Option<Box<Class>>
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
            inherited_methods: self.inherited_methods.clone(),
            constructor: self.constructor.clone(),
            super_class: self.super_class.clone()
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
        let mut instance = ClassInstance {
            class: Rc::new(self.clone()),
            attributes: Rc::new(RefCell::new(HashMap::new())) 
        };
        let current_class = Box::new(self.clone());

        self.set_methods_on_instance(&mut instance, &current_class);
        if let Some(_) = &self.constructor {
            let func_obj = instance.get(&String::from("init")).unwrap();
            let init_method = func_obj.as_function();
            if let Some(init) = init_method {
                init.call(params, out_func_state, line);
            }
        }
        instance
    }

    fn set_method_on_instance(instance: &mut ClassInstance, func_stmt: &FunctionDeclStatement) {
        let instance_copy: Box<dyn Object> = Box::new(instance.clone());
        let mut func_copy = func_stmt.function_decl.clone();
        if func_stmt.extern_variables.iter().any(|ident| ident.value == "this") {
            func_copy.extra_map.insert(String::from("this"), Rc::new(RefCell::new(instance_copy)));
        }
        instance.set(&func_stmt.function_decl.name, Box::new(func_copy));
    }

    fn set_method_on_inherit_instance(&self, instance: &mut ClassInstance, parent_class: &Box<Class>, func_stmt: &FunctionDeclStatement) {
        let instance_copy: Box<dyn Object> = Box::new(instance.clone());
        let mut func_copy = func_stmt.function_decl.clone();
        if func_stmt.extern_variables.iter().any(|ident| ident.value == "this") {
            func_copy.extra_map.insert(String::from("this"), Rc::new(RefCell::new(instance_copy)));
        }
        if func_stmt.extern_variables.iter().any(|ident| ident.value == "super") {
            let mut parent_instance = ClassInstance {
                class: Rc::new(*parent_class.clone()),
                attributes: Rc::new(RefCell::new(HashMap::new())) 
            };   
            self.set_methods_on_instance(&mut parent_instance, parent_class);
            let parent_obj: Box<dyn Object> = Box::new(parent_instance.clone());
            func_copy.extra_map.insert(String::from("super"), Rc::new(RefCell::new(parent_obj)));
        }
        instance.set(&func_stmt.function_decl.name, Box::new(func_copy));
    }

    fn set_methods_on_instance(&self, instance: &mut ClassInstance, class: &Box<Class>) {
        if let Some(superclass) = &class.super_class {
            let mut parent_instance = ClassInstance {
                class: Rc::new(*superclass.clone()),
                attributes: Rc::new(RefCell::new(HashMap::new())) 
            };   
            self.set_methods_on_instance(&mut parent_instance, superclass);
            for (_, func_stmt) in class.methods.iter() {
                self.set_method_on_inherit_instance(instance, superclass, &func_stmt);
            }
            if let Some(init_method_stmt) = &class.constructor {
                self.set_method_on_inherit_instance(instance, superclass,&init_method_stmt);
            }
            for (funcname, func_stmt) in class.inherited_methods.iter() {
                let mut parent_class = Some(superclass.clone());
                while let Some(super_class) = &parent_class {
                    if super_class.inherited_methods.contains_key(funcname) {
                        parent_class = super_class.super_class.clone();
                        continue;
                    }
                    self.set_method_on_inherit_instance(instance, superclass, &func_stmt);
                    break;
                }
            }
        }
        else {
            for (_, func_stmt) in class.methods.iter() {
                Class::set_method_on_instance(instance,  &func_stmt);
            }
            if let Some(init_method_stmt) = &class.constructor {
                Class::set_method_on_instance(instance, &init_method_stmt);
            }
        }
    }
}


impl Expression for InstanceGetSetExpr {
    fn evaluate(&self, state_scope: &mut BlockScopes) -> Box<dyn Object> {
        let mut obj = self.instance.evaluate(state_scope);
        if obj.get_type() != Type::CLASSINSTANCE {
            handle_error(&self.line, ErrorType::RuntimeError, 
                "Can only access property on class instance");
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
            }
            return prop.unwrap();
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
