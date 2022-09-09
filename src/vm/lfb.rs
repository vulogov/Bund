extern crate log;
extern crate pest;
use crate::lang::Rule;
use crate::vm::vm;

pub fn process_token(b: &mut vm::VM, p: &pest::iterators::Pair<Rule>, _t: &String) {
    log::debug!("Received LFB token: {:#?}", p.as_rule());
    b.new_stack();
    let mut v = b.value().unwrap();
    v.has_attr = true;
    b.add_value(v);
}
