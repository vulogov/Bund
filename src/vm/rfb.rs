extern crate log;
extern crate pest;
use crate::lang::Rule;
use crate::vm::vm;

pub fn process_token(b: &mut vm::VM, p: &pest::iterators::Pair<Rule>, _t: &String) {
    log::debug!("Received RFB token: {:#?}", p.as_rule());
    let attr = b.take_stack();
    b.to_attr(attr)
}
