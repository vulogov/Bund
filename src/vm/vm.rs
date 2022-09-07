extern crate log;
use std::collections;
use crate::twostack;
use crate::twostack::value;
use crate::vm::bundfunction;


pub struct VM {
    v:          collections::VecDeque<value::Value>,
    ts:         twostack::TS,
    functions:  collections::HashMap<String,bundfunction::BundFunction>,
}

impl VM {
    pub fn new() -> Self {
        log::debug!("Creating BUND VM");
        Self {
            ts:             twostack::TS::new(),
            v:              collections::VecDeque::new(),
            functions:      collections::HashMap::new(),
        }
    }
}

impl VM {
    pub fn local(&mut self) -> &mut collections::VecDeque<value::Value> {
        self.ts.local()
    }
    pub fn get(&mut self) -> Option<&value::Value> {
        self.ts.get()
    }
    pub fn set(&mut self, v: value::Value)  {
        self.ts.set(v)
    }
    pub fn drop_function(&mut self, name: &String) -> Option<bundfunction::BundFunction> {
        self.functions.remove(name)
    }
    pub fn add_function(&mut self, n: &String, f: bundfunction::BundFunction) {
        log::debug!("Registering BUND function: {:?}", &n);
        self.drop_function(&n);
        self.functions.insert(n.to_string(), f);
    }
    pub fn function(&self, name: &String) -> Option<&bundfunction::BundFunction> {
        self.functions.get(name)
    }
    pub fn value(&mut self) -> Option<value::Value> {
        if self.v.is_empty() {
            return None;
        }
        log::trace!("Taking value from value cache");
        self.v.pop_back()
    }
    pub fn add_value(&mut self, v: value::Value) {
        log::trace!("Adding value from lexer: {:?}", &v);
        self.v.push_back(v)
    }
}

impl VM {
    pub fn new_stack(&mut self) -> &mut collections::VecDeque<value::Value> {
        self.ts.new_stack()
    }
    pub fn new_named_stack(&mut self, n: &String) -> &mut collections::VecDeque<value::Value> {
        self.ts.new_named_stack(n)
    }
}
