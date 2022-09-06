use std::collections;
pub mod value;

pub struct TS {
    stack:       collections::VecDeque<collections::VecDeque<value::Value>>,
}

impl TS {
    pub fn new() -> Self {
        Self {
            stack: collections::VecDeque::new(),
        }
    }
}

impl TS {
    fn if_empty(&mut self) {
        if self.stack.is_empty() {
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
}
