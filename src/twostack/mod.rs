extern crate log;
use std::collections;
use crate::stdlib::genid;
pub mod value;

pub struct TS {
    names:       collections::VecDeque<String>,
    stack:       collections::VecDeque<collections::VecDeque<value::Value>>,
}

impl TS {
    pub fn new() -> Self {
        Self {
            stack: collections::VecDeque::new(),
            names: collections::VecDeque::new(),
        }
    }
}

impl TS {
    fn if_empty(&mut self) {
        if self.stack.is_empty() {
            let id = genid::generate_id();
            self.names.push_back(id);
            self.stack.push_back(collections::VecDeque::new())
        }
    }
    pub fn global(&mut self) -> &mut collections::VecDeque<collections::VecDeque<value::Value>> {
        let _ = &mut self.if_empty();
        &mut self.stack
    }
    pub fn local(&mut self) -> &mut collections::VecDeque<value::Value> {
        let _ = &mut self.if_empty();
        self.stack.back_mut().unwrap()
    }
    pub fn take_stack(&mut self) -> collections::VecDeque<value::Value> {
        let _ = &mut self.if_empty();
        let name = self.names.pop_back().unwrap();
        log::trace!("Taking stack: {:?}", &name);
        self.stack.pop_back().unwrap()
    }
    pub fn new_stack(&mut self) -> &mut collections::VecDeque<value::Value> {
        let id = genid::generate_id();
        log::trace!("Create new stack: {:?}", &id);
        self.names.push_back(id);
        let _ = self.stack.push_back(collections::VecDeque::new());
        self.stack.back_mut().unwrap()
    }
    pub fn new_named_stack(&mut self, n: &String) -> &mut collections::VecDeque<value::Value> {
        if self.names.contains(n) {
            self.position(n);
            return self.local();
        }
        log::trace!("Create new stack: {:?}", &n);
        self.names.push_back(n.to_string());
        let _ = self.stack.push_back(collections::VecDeque::new());
        self.stack.back_mut().unwrap()
    }
    pub fn drop_stack(&mut self)  {
        let _ = &mut self.if_empty();
        let _ = &mut self.stack.pop_back();
        let name = &self.current();
        let _ = &mut self.names.pop_back();
        let _ = &mut self.if_empty();
        log::trace!("Dropping stack: {:?}", name);
    }
    pub fn get(&mut self) -> Option<&value::Value> {
        let _ = &mut self.if_empty();
        self.local().back()
    }
    pub fn set(&mut self, v: value::Value) {
        let _ = &mut self.if_empty();
        let name  = &self.current();
        let local = &mut self.local();
        log::trace!("Push {:?} to {:?}", &v, &name);
        local.push_back(v)
    }
    pub fn drop(&mut self) {
        let _ = &mut self.if_empty();
        let local = &mut self.local();
        match local.back() {
            Some(_) => &mut local.pop_back(),
            None => return,
        };
    }
    pub fn dup(&mut self) {
        let _ = &mut self.if_empty();
        let local = &mut self.local();
        match local.back() {
            Some(v) => local.push_back(v.clone()),
            None => return,
        };
    }
    pub fn current(&self) -> String {
        self.names.back().unwrap().to_string()
    }
    pub fn is_current(&self, name: &String) -> bool {
        if self.names.back().unwrap().to_string() == *name {
            return true;
        }
        return false;
    }
    pub fn global_left(&mut self) -> &mut collections::VecDeque<value::Value> {
        let _ = &mut self.if_empty();
        let _ = &mut self.names.rotate_left(1);
        let _ = &mut self.stack.rotate_left(1);
        self.local()
    }
    pub fn global_right(&mut self) -> &mut collections::VecDeque<value::Value> {
        let _ = &mut self.if_empty();
        let _ = &mut self.names.rotate_right(1);
        let _ = &mut self.stack.rotate_right(1);
        self.local()
    }
    pub fn local_right(&mut self) -> Option<&value::Value> {
        let _ = &mut self.if_empty();
        let local = &mut self.local();
        let _ = local.rotate_right(1);
        self.get()
    }
    pub fn local_left(&mut self) -> Option<&value::Value> {
        let _ = &mut self.if_empty();
        let local = &mut self.local();
        let _ = local.rotate_left(1);
        self.get()
    }
    pub fn position(&mut self, name: &String) -> bool {
        let _ = &mut self.if_empty();
        let last = &mut self.current();
        if last == name {
            return true;
        }
        loop {
            let _ = &mut self.global_left();
            if last == &mut self.current() {
                return false;
            }
            if last == name {
                break;
            }
        }
        return true;
    }
}
