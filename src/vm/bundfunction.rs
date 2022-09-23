use std::collections;
use std::error::Error;
use crate::vm::vm;
use crate::twostack::value;

type BundFunctionPtr = fn(&mut vm::VM, &mut collections::VecDeque<value::Value>);

pub struct BundFunction {
    name:       String,
    min_attr:   i32,
    fun_ptr:    BundFunctionPtr,
}

impl BundFunction {
    pub fn new(name: &str, n: i32, ptr: BundFunctionPtr) -> Self {
        Self {
            name:           name.to_string(),
            min_attr:       n as i32,
            fun_ptr:        ptr,
        }
    }
}

impl BundFunction {
    pub fn n(&self) -> Result<i32, Box<dyn Error>> {
        Ok(self.min_attr)
    }
    pub fn f(&self) -> Result<BundFunctionPtr, Box<dyn Error>> {
        Ok(self.fun_ptr)
    }
    pub fn name(&self) -> Result<&String, Box<dyn Error>> {
        Ok(&self.name)
    }
}
