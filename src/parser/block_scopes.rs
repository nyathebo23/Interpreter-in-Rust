use std::collections::HashMap;

use crate::function_manage::Function;
use crate::parser::declarations::Object;


pub struct BlockScopes {
    vars_nodes_map: Vec<HashMap<String, Box<dyn Object>>>,
    pub func_declarations: HashMap<String, Function>,
    depth: usize
}

impl BlockScopes {
    pub fn new(functions_map: HashMap<String, Function>) -> BlockScopes {
        BlockScopes { 
            vars_nodes_map: Vec::from([
                HashMap::new(),
            ]),
            func_declarations: functions_map,
            depth: 0
        }
    }

    // pub fn copy(&self) -> BlockScopes {
    //     let mut vars_map_list = Vec::new();
    //     for map in self.vars_nodes_map.iter() {
    //         let mut var_hashmap = HashMap::new();
    //         for (key, val) in map.iter() {
    //             var_hashmap.insert(key.clone(), val.dyn_clone());
    //         }
    //         vars_map_list.push(var_hashmap);
    //     }
    //     let mut functions_decls = HashMap::new();
    //     for (key, val) in self.func_declarations.iter() {
    //         functions_decls.insert(key.clone(), val.duplicate_func_scope());
    //     }

    //     BlockScopes { 
    //         vars_nodes_map: vars_map_list, 
    //         func_declarations: functions_decls, 
    //         depth: self.depth
    //     }
    // }

    pub fn set_global_variable(&mut self, identifier: &String, value: Box<dyn Object>) {
        match self.vars_nodes_map.get_mut(0) {
            Some(node_map) => {
                node_map.insert(identifier.to_string(), value);
            },
            None => {}
        };
    }

    pub fn get_global_variable(&mut self, identifier: &String) -> Option<Box<dyn Object>> {
        if let Some(hashmap) = self.vars_nodes_map.first() {
            match hashmap.get(identifier) {
                Some(value) => { return Some(value.dyn_clone()); },
                None => {}
            } 
        }
        None
    }

    pub fn define_function(&mut self, func_name: String, function: Function) {
        self.func_declarations.insert(func_name, function);
    }

    pub fn get_func(&mut self, func_name: &String) -> Option<Function> {
        match self.func_declarations.get(func_name) {
            Some(func) => Some(func.duplicate_func_scope()),
            None => None
        }
    }

    pub fn start_child_block(&mut self) {
        self.vars_nodes_map.push(HashMap::new());
        self.depth += 1;
    }

    pub fn end_child_block(&mut self) {
        self.vars_nodes_map.pop();
        self.depth -= 1;
    }

    pub fn set_init_variable(&mut self, identifier: &String, value: Box<dyn Object>) {
        match self.vars_nodes_map.get_mut(self.depth) {
            Some(node_map) => {
                node_map.insert(identifier.to_string(), value);
            },
            None => {}
        };
    }

    pub fn modif_variable(&mut self, identifier: &String, new_value: Box<dyn Object>) {
        for hashmap in self.vars_nodes_map.iter_mut().rev() {
            match hashmap.get(identifier) {
                Some(_value) => { 
                    hashmap.insert(identifier.to_string(), new_value.dyn_clone());    
                },
                None => {}
            } 
        }
    }

    pub fn get_variable(&mut self, identifier: &String) -> Option<Box<dyn Object>> {
        for hashmap in self.vars_nodes_map.iter().rev() {
            match hashmap.get(identifier) {
                Some(value) => { return Some(value.dyn_clone()); },
                None => {}
            } 
        }
        None
    }
}
