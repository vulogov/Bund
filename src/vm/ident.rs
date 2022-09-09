extern crate log;
extern crate pest;
use crate::lang::Rule;
use crate::vm::vm;

pub fn process_token(b: &mut vm::VM, p: &pest::iterators::Pair<Rule>, t: &String) {
    log::debug!("Received IDENT token: {:#?}({})", p.as_rule(), t);
    let mut v = b.value().unwrap();
    v.call(t);
    b.add_value(v);
}
