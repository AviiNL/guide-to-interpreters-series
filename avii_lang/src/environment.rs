use std::collections::HashMap;

use crate::{interpreter::{RuntimeVal, BuiltInFunction}};

#[derive(Debug, Clone)]
pub struct Environment {
    parent: Option<Box<Environment>>,
    pub(crate) variables: HashMap<String, RuntimeVal>,
    constants: Vec<String>,
}

impl Environment {
    pub fn new() -> Self {
        let mut env = Environment {
            parent: None,
            variables: HashMap::new(),
            constants: Vec::new(),
        };

        env.set("true", RuntimeVal::BoolVal(true), true);
        env.set("false", RuntimeVal::BoolVal(false), true);
        env.set("null", RuntimeVal::NullVal, true);

        env.set("print", RuntimeVal::BuiltInFunction(BuiltInFunction {
            function: Box::new(|args| {
                print!("{}", args[0]);
                RuntimeVal::NullVal
            }),
        }), true);

        env
    }

    pub fn new_with_parent(parent: Environment) -> Self {
        Environment {
            parent: Some(Box::new(parent)),
            variables: HashMap::new(),
            constants: Vec::new(),
        }
    }

    pub fn with_default_scope(mut self) -> Self {
        self.variables.insert("PI".to_string(), RuntimeVal::NumberVal(std::f64::consts::PI));
        self
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
            Some(e) => {
                let symbol = symbol.to_owned();

                if e.constants.contains(&symbol) {
                    panic!("Cannot assign to constant {}", symbol);
                }

                e.variables.insert(symbol.clone(), value.clone());

                value
            },
            None => panic!("Variable {} not defined", symbol),
        }
    }

    pub fn resolve(&mut self, symbol: &str) -> Option<&mut Environment> {
        if self.variables.contains_key(symbol) {
            return Some(self);
        }

        match &mut self.parent {
            Some(p) => p.resolve(symbol),
            None => None,
        }
    }

    pub fn get(&mut self, symbol: &str) -> Option<RuntimeVal> {
        let env = self.resolve(symbol);

        match env {
            Some(e) => e.variables.get(symbol).map(|v| v.clone()),
            None => None,
        }
    }

}