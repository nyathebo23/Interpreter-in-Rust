use crate::{error_handler::{handle_error, ErrorType}, parser::expressions::Identifier};
use std::mem;


#[derive(PartialEq, Clone)]
pub enum FunctionType {
    NONE,
    FUNCTION,
    CLASSFUNCTION,
    INITCLASSFUNC
}

#[derive(PartialEq, Clone)]
pub enum ClassType {
    NONE,
    CLASS,
    CHILDCLASS
}

#[derive(Clone)]
struct Node {
    pub current_function: FunctionType,
    pub current_class: ClassType, 
    pub declarations: Vec<String>,
    pub out_identifiers: Vec<Identifier>,
    pub parent: Option<Box<Node>>,
}


impl Node {
    pub fn init() -> Node {
        Node {
            current_class: ClassType::NONE,
            current_function: FunctionType::NONE,
            out_identifiers: Vec::new(),
            declarations: Vec::new(),
            parent: None
        }
    }

    pub fn new_class(&mut self, class: ClassType, class_name: &String)  {
        self.declarations.push(class_name.clone());
        *self = Node {
            current_class: class,
            current_function: FunctionType::NONE,
            out_identifiers: Vec::new(),
            declarations: Vec::new(),
            parent: Some(Box::new(self.clone()))
        }
    }

    pub fn end_class(&mut self)  {
        if let Some(parent) = &self.parent {
            if self.current_function == FunctionType::NONE {
                *self = *parent.clone();
            }
        }
    }

    pub fn new_func(&mut self, func_name: &String) {
        self.declarations.push(func_name.clone());
        *self = Node {
            current_class: self.current_class.clone(),
            current_function: FunctionType::FUNCTION,
            out_identifiers: Vec::new(),
            declarations: Vec::new(),
            parent: Some(Box::new(self.clone()))
        }
    }

    pub fn end_func(&mut self) -> Vec<Identifier> {
        let out_decls =  mem::take(&mut self.out_identifiers);
        if let Some(parent) = &self.parent {
            *self = *parent.clone();
        }
        return out_decls;
    }

    pub fn new_block(&mut self) {
        *self = Node {
            current_class: self.current_class.clone(),
            current_function: self.current_function.clone(),
            out_identifiers: Vec::new(),
            declarations: Vec::new(),
            parent: Some(Box::new(self.clone()))
        }
    }

    pub fn end_block(&mut self) -> Vec<Identifier>  {
        let out_decls =  mem::take(&mut self.out_identifiers);
        if let Some(parent) = &self.parent {
            *self = *parent.clone();
        }
        return out_decls;
    }

    pub fn start_init_class_func(&mut self) {
        self.current_function = FunctionType::INITCLASSFUNC;
    }

    pub fn new_class_func(&mut self) {
        self.current_function = FunctionType::CLASSFUNCTION;
    }

    pub fn end_class_func(&mut self) -> Vec<Identifier> {
        let identifiers =  mem::take(&mut self.out_identifiers);
        self.current_function = FunctionType::NONE;
        identifiers
    }

}

pub struct Environment {
    nodes_tree: Node,
}

impl Environment {
    pub fn new() -> Environment {
        Environment { 
            nodes_tree: Node::init(),
        }
    }



    pub fn check_identifiers(&mut self, identifiers: Vec<Identifier>, expr_line: u32) {
        for ident in identifiers {
            let ident_str = ident.value.clone();
            if ident_str == "this" {
                if self.nodes_tree.current_class == ClassType::NONE {
                    Environment::compile_keyword_class_err(expr_line, &ident_str);
                }
                else {
                    self.nodes_tree.out_identifiers.push(ident.clone());
                }
            }
            else if ident_str == "super" {
                if self.nodes_tree.current_class == ClassType::NONE {
                    Environment::compile_keyword_class_err(expr_line, &ident_str);
                }
                else if self.nodes_tree.current_class != ClassType::CHILDCLASS {
                    Environment::compile_bad_class_super_err(expr_line);
                }
                else {
                    self.nodes_tree.out_identifiers.push(ident.clone());
                }
            }
            else {
                if !self.nodes_tree.declarations.contains(&ident_str) {
                    self.nodes_tree.out_identifiers.push(ident.clone());
                }
            }
        }

    }

    pub fn declaration(&mut self, var_name: &String, line: &u32, expr_identifiers: Vec<Identifier>, expr_line: u32) {

        let decls = &self.nodes_tree.declarations;
        if let Some(_) = &self.nodes_tree.parent {
            if decls.contains(var_name) {
                handle_error(line, ErrorType::SyntacticError, 
                format!("Error at {}: Already a variable with this name in this scope.", var_name.clone()).as_str());
            }
            if expr_identifiers.iter().any(|ident| ident.value == *var_name ) {
                handle_error(line, ErrorType::SyntacticError, 
                    format!("Error at {}: Can't read local variable in its own initializer.", var_name.clone()).as_str());
            }
        }
        self.check_identifiers(expr_identifiers, expr_line);
        self.nodes_tree.declarations.push(var_name.clone());
        
    }

    pub fn set_func_params(&mut self, params: &Vec<String>) {
        self.nodes_tree.declarations = params.clone();
    }

    pub fn start_block(&mut self) {
        self.nodes_tree.new_block();
    }

    pub fn end_block(&mut self) {
        let idents = self.nodes_tree.end_block();
        for item in idents {
            if !self.nodes_tree.declarations.contains(&item.value) {
                self.nodes_tree.out_identifiers.push(item);
            }
        }
    }

    pub fn start_function(&mut self, funcname: &String) {
        self.nodes_tree.new_func(funcname);
    }

    pub fn end_function(&mut self) -> Vec<Identifier> {
        let idents = self.nodes_tree.end_func();
        if self.nodes_tree.current_class == ClassType::NONE {
            return idents;
        }
        for item in &idents {
            if (item.value == "this" || item.value == "super") && 
                !self.nodes_tree.out_identifiers.iter().any(|ident| ident.value == item.value) {
                self.nodes_tree.out_identifiers.push(item.clone());
            }
        }
        idents
    }

    pub fn start_class(&mut self, classname: &String) {
        self.nodes_tree.new_class(ClassType::CLASS, classname);
    }

    pub fn end_class(&mut self) {
        self.nodes_tree.end_class();
    }

    pub fn start_class_func(&mut self) {
        self.nodes_tree.new_class_func();
    }

    pub fn end_class_func(&mut self) -> Vec<Identifier>  {
        self.nodes_tree.end_class_func()
    }

    pub fn start_init_class_func(&mut self) {
        self.nodes_tree.start_init_class_func();
    }

    pub fn start_child_class(&mut self, classname: &String) {
        self.nodes_tree.new_class(ClassType::CHILDCLASS, classname);
    }

    pub fn check_return_validity(&self, line: &u32) {
        if self.nodes_tree.current_function == FunctionType::NONE {
            handle_error(&line, ErrorType::SyntacticError, 
                "Error at 'return': Can't return from top-level code.");
        }
    }

    pub fn check_constructor_return_validity(&self, line: &u32) {
        if self.nodes_tree.current_function == FunctionType::INITCLASSFUNC {
            handle_error(line, ErrorType::SyntacticError, 
                "Error at 'return': Can't return a value from an initializer");
        }
    }

    fn compile_bad_class_super_err(line: u32) {
        handle_error(&line, ErrorType::SyntacticError, 
    "Error at 'super': Can't use 'super' in a class with no superclass");
    }

    fn compile_keyword_class_err(expr_line: u32, keyword: &str) {
        handle_error(&expr_line, ErrorType::SyntacticError, 
        format!("Error at '{}': Can't use '{}' outside of a class.", keyword, keyword).as_str());
    }

}