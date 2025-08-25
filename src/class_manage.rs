use std::{borrow::Cow, rc::Rc};

use crate::parser::{block_scopes::BlockScopes, declarations::{Object, Type, ValueObjTrait}, expressions::Expression};


#[derive(Clone)]
pub struct Class {
    pub name: String,
}


#[derive(Clone)]
pub struct ClassInstance {
    pub class: Rc<Class>
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
            class: self.class.clone()
        })
    } 
}


impl ValueObjTrait for Class {
    fn as_class(&self) -> Option<&Class> {
        Some(self)
    }
}

impl ValueObjTrait for ClassInstance  {
    fn as_class_instance(&self) -> Option<&ClassInstance> {
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
            class: Rc::new(self.clone())
        };
        instance
    }
}