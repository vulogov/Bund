extern crate log;
extern crate pest;
use crate::lang::Rule;
use crate::vm::vm;

pub fn process_token(b: &mut vm::VM, p: &pest::iterators::Pair<Rule>, t: &String) {
    log::debug!("Received STRING token: {:#?}", p.as_rule());
    let mut v = b.value().unwrap();
    v.string(t);
    b.add_value(v);
}
