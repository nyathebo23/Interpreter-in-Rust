
use crate::class::ClassInstance;
use crate::interpreter::block_scopes::BlockScopes;
use crate::parser::declarations::*;
use crate::parser::operators_decl::*;


pub trait Expression {
    fn evaluate(&self, state_scope: &mut BlockScopes) -> Box<dyn Object>;
    fn contains_identifier(&self, ident: &String) -> bool;
    fn to_string(&self) -> String;
    fn value_from_class_instance(&self, instance: &ClassInstance, state_scope: &mut BlockScopes) -> (String, Box<dyn Object>);
}

pub struct InstanceGetSetExpr {
    pub instance: Box<dyn Expression>,
    pub property: Box<dyn Expression>,
    pub value_to_assign: Option<Box<dyn Expression>>,
    pub line: u32 
}

impl InstanceGetSetExpr {
    pub fn new(instance: Box<dyn Expression>, property: Box<dyn Expression>, 
        value_to_assign: Option<Box<dyn Expression>>, line: u32) -> InstanceGetSetExpr {
            InstanceGetSetExpr { 
                instance, 
                property, 
                value_to_assign, 
                line
            }
        }
}


pub struct CallExpr {
    pub callable: Box<dyn Expression>,
    pub params: Vec<Box<dyn Expression>>,
    pub line: u32
}

impl CallExpr {
    pub fn new(callable: Box<dyn Expression>, params: Vec<Box<dyn Expression>>, line: u32) -> CallExpr {
        CallExpr { 
            callable, 
            params, 
            line
        }
    }
}

pub struct IdentifierExpr {
    pub ident_name: String,
    pub value_to_assign: Option<Box<dyn Expression>>,
    pub line: u32
}

impl IdentifierExpr {
    pub fn new(ident: String, value: Option<Box<dyn Expression>>, line: u32) -> IdentifierExpr {
        IdentifierExpr { 
            ident_name: ident, 
            value_to_assign: value, 
            line 
        }
    }
}

pub struct BinaryExpr {
    pub operator: BinaryOperator,
    pub value1: Box<dyn Expression>,
    pub value2: Box<dyn Expression>,
    pub line: u32
}

impl BinaryExpr {
    pub fn new(op: BinaryOperator, val1: Box<dyn Expression>, val2: Box<dyn Expression>, line: u32) -> BinaryExpr {
        BinaryExpr { 
            operator: op, 
            value1: val1, 
            value2: val2, 
            line 
        }
    }
}

pub struct UnaryExpr {
    pub operator: UnaryOperator,
    pub value: Box<dyn Expression>,
    pub line: u32
}

impl UnaryExpr {
    pub fn new(operator: UnaryOperator, value: Box<dyn Expression>, line: u32) -> UnaryExpr {
        UnaryExpr {
            operator,
            value,
            line
        }
    }
}
pub struct LiteralExpr {
    pub value: Box<dyn Object>,
}

impl LiteralExpr {
    pub fn new(value: Box<dyn Object>) -> LiteralExpr {
        LiteralExpr { value }
    }
}

pub struct GroupExpr  {
    pub value: Box<dyn Expression>,
}

impl GroupExpr {
    pub fn new(value: Box<dyn Expression>) -> GroupExpr {
        GroupExpr { value }
    }
}

