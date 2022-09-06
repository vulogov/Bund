use std::collections;
use crate::twostack;
use crate::twostack::value;
use crate::vm::bundfunction;


pub struct VM {
    ts:         twostack::TS,
    functions:  collections::HashMap<String,bundfunction::BundFunction>,
}

impl VM {
    pub fn new() -> Self {
        Self {
            ts:             twostack::TS::new(),
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
    pub fn drop_function(&mut self, name: &String) -> Option<bundfunction::BundFunction> {
        self.functions.remove(name)
    }
}
