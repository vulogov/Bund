use std::collections;
use crate::twostack::value;

pub struct BundContext {
    ctx:    collections::VecDeque<collections::HashMap<String,value::Value>>,
}

impl BundContext {
    pub fn new() -> Self {
        Self {
            ctx:           collections::VecDeque::new(),
        }
    }
}

impl BundContext {
    fn if_empty(&mut self) {
        if self.ctx.is_empty() {
            self.ctx.push_back(collections::HashMap::new())
        }
    }
    pub fn current(&mut self) -> &mut collections::HashMap<String,value::Value> {
        let _ = &mut self.if_empty();
        self.ctx.back_mut().unwrap()
    }
    pub fn set(&mut self, name: String, v: value::Value) {
        let _ = &mut self.if_empty();
        let c = &mut self.current();
        c.insert(name, v);
    }
    pub fn set_by_ref(&mut self, name: String, v: &value::Value) {
        let _ = &mut self.if_empty();
        let c = &mut self.current();
        c.insert(name, v.clone());
    }
    pub fn get(&mut self) -> Option<value::Value> {
        let _ = &mut self.if_empty();
        None
    }
}
