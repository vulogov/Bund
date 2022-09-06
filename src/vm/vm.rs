use std::collections;
use crate::twostack;

type BundFunctionPtr = fn(&VM, &BundFunction);

pub struct BundFunction {
    name:       String,
    min_attr:   i32,
    fun_ptr:    BundFunctionPtr,
}

pub struct VM {
    ts:         twostack::TS,
    functions:  collections::HashMap<String,BundFunction>,
}

impl VM {
    pub fn new() -> Self {
        Self {
            ts:             twostack::TS::new(),
            functions:      collections::HashMap::new(),
        }
    }
}
