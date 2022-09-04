extern crate log;
extern crate pest;
use crate::lang::Rule;
use crate::vm::vm;

pub fn process_token(_b: &vm::VM, p: &pest::iterators::Pair<Rule>, t: &String) {
    log::debug!("Received SUFFIX token: {:#?}({})", p.as_rule(), t);
}
