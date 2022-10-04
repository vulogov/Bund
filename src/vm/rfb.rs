extern crate log;
extern crate pest;
use crate::lang::Rule;
use crate::vm::vm;

pub fn process_token(b: &mut vm::VM, p: &pest::iterators::Pair<Rule>, _t: &String) {
    let v = b.value().unwrap();
    log::debug!("Received RFB token: {:#?} for {:?}", p.as_rule(), &v.as_string());
    let attr = b.take_stack();
    b.to_attr(attr);
    b.add_value(v);
}
