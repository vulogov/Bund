use std::collections;

pub struct BundContext {
    ctx:    collections::VecDeque<collections::HashMap<String,String>>,
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
    pub fn current(&mut self) -> &mut collections::HashMap<String,String> {
        let _ = &mut self.if_empty();
        self.ctx.back_mut().unwrap()
    }
    pub fn set(&mut self, name: String, v: String) {
        let _ = &mut self.if_empty();
        let c = &mut self.current();
        c.remove(&v);
        c.insert(name, v);
    }
    pub fn set_by_ref(&mut self, name: String, v: &String) {
        let _ = &mut self.if_empty();
        let c = &mut self.current();
        c.remove(v);
        c.insert(name, v.clone());
    }
    pub fn get(&mut self, n: String) -> Option<&String> {
        let _ = &self.if_empty();
        let m = &mut self.ctx.len();
        while m >= &mut 0 {
            *m -= 1;
            let c = &self.ctx.get(*m).unwrap();
            if c.contains_key(&n) {
                return c.get(&n);
            }
        }
        None
    }
}
