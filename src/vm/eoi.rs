extern crate log;
extern crate pest;
use crate::lang::Rule;
use crate::vm::vm;

pub fn process_token(_b: &mut vm::VM, p: &pest::iterators::Pair<Rule>, _t: &String) {
    log::debug!("Received EOI token: {:#?}", p.as_rule());
}
