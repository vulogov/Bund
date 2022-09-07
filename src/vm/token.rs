extern crate log;
extern crate pest;
use crate::lang::Rule;
use crate::twostack::value;
use crate::vm::vm;

pub fn process_token(b: &mut vm::VM, p: &pest::iterators::Pair<Rule>, t: &String) {
    log::debug!("Received TOKEN token: {:#?}({})", p.as_rule(), t);
    b.add_value(value::Value::new());
    b.new_stack();
}

pub fn post_process_token(b: &mut vm::VM, r: &Rule, t: &String) {
    log::debug!("TOKEN postprocessing: {:?}({})", &r, &t);
    let v   = b.value().unwrap();
    match v.type_of() {
        value::NONE ..=value::LITERAL => {
            b.set(v);
            post_process_data_token(b);
        }
        _ => todo!(),
    }
}

fn post_process_data_token(b: &mut vm::VM) {
    let mut attr = b.take_stack();
    while ! attr.is_empty() {
        let v = attr.pop_front().unwrap();
        b.set(v);
    }
}
