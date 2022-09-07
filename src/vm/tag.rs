extern crate log;
extern crate pest;
use crate::lang::Rule;
use crate::vm::vm;

pub fn process_token(b: &mut vm::VM, p: &pest::iterators::Pair<Rule>, t: &String) {
    log::debug!("Received TAG token: {:#?}({})", p.as_rule(), t);
    let mut v = b.value().unwrap();
    let _ = v.set_tag(t);
    b.add_value(v);
}
