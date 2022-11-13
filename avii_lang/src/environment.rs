use std::{collections::HashMap, sync::{Arc, RwLock}};

use crate::interpreter::RuntimeVal;

#[derive(Debug, Clone)]
pub struct Environment {
    parent: Option<Arc<RwLock<Environment>>>,
    pub(crate) variables: HashMap<String, RuntimeVal>,
    constants: Vec<String>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            parent: None,
            variables: HashMap::new(),
            constants: Vec::new(),
        }
    }

    pub fn new_with_parent(parent: Arc<RwLock<Environment>>) -> Self {
        Environment {
            parent: Some(parent),
            variables: HashMap::new(),
            constants: Vec::new(),
        }
    }

    pub fn set(&mut self, symbol: &str, value: RuntimeVal, is_const: bool) -> RuntimeVal {

        if self.variables.contains_key(symbol) {
            panic!("Variable {} already defined", symbol);
        }
        
        self.variables.insert(symbol.to_string(), value.clone());

        if is_const {
            self.constants.push(symbol.to_string());
        }

        value
    }

    pub fn assign(&mut self, symbol: &str, value: RuntimeVal) -> RuntimeVal {

        let env = self.resolve(symbol);

        match env {
            Some(mut e) => {
                let symbol = symbol.to_owned();

                if e.constants.contains(&symbol) {
                    panic!("Cannot assign to constant {}", symbol);
                }

                e.variables.insert(symbol, value.clone());
                value
            },
            None => panic!("Variable {} not defined", symbol),
        }
    }

    pub fn resolve(&self, symbol: &str) -> Option<Environment> {
        if self.variables.contains_key(symbol) {
            return Some(self.clone());
        }

        if let Some(parent) = &self.parent {
            return parent.read().unwrap().resolve(symbol);
        }

        None
    }

    pub fn get(&self, symbol: &str) -> Option<RuntimeVal> {

        let env = self.resolve(symbol);

        match env {
            Some(e) => e.variables.get(symbol).map(|v| v.clone()),
            None => None,
        }
    }

}