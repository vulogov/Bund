extern crate log;
extern crate pest;
use crate::lang::Rule;
use crate::vm::vm;

pub fn process_token(b: &mut vm::VM, p: &pest::iterators::Pair<Rule>, _t: &String) {
    let mut v = b.value().unwrap();
    log::debug!("Received LFB token: {:#?} for {:?}", p.as_rule(), &v.as_string());
    b.begin_codeblock(&v.as_string().unwrap());
    b.new_stack();
    v.has_attr = true;
    b.add_value(v);
}
