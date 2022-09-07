extern crate log;
extern crate pest;
use crate::lang::Rule;
use crate::twostack::value;
use crate::vm::vm;

pub fn process_token(b: &mut vm::VM, p: &pest::iterators::Pair<Rule>, t: &String) {
    log::debug!("Received TOKEN token: {:#?}({})", p.as_rule(), t);
    b.add_value(value::Value::new());
}

pub fn post_process_token(_b: &vm::VM, r: &Rule, t: &String) {
    log::debug!("TOKEN postprocessing: {:?}({})", &r, &t);
}
