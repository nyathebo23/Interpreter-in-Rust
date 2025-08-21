use std::collections::HashMap;

use crate::parser::declarations::Object;

pub struct BlockScopes {
    vars_nodes_map: Vec<HashMap<String, Box<dyn Object>>>,
    depth: usize
}

impl BlockScopes {
    pub fn new() -> BlockScopes {
        BlockScopes { 
            vars_nodes_map: Vec::from([
                HashMap::new(),
            ]),
            depth: 0
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
