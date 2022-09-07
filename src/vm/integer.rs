extern crate log;
extern crate pest;
use crate::lang::Rule;
use crate::vm::vm;

pub fn process_token(b: &mut vm::VM, p: &pest::iterators::Pair<Rule>, t: &String) {
    log::debug!("Received INTEGER token: {:#?}({})", p.as_rule(), t);
    let num: Result<i64, lexical_core::Error> = lexical_core::parse(t.as_bytes());
    match num {
        Ok(val) => {
            let mut v = b.value().unwrap();
            v.int(val);
            b.add_value(v);
        }
        Err(err) => {
            log::error!("Error parsing INTEGER token: {:?}", err);
        }
    }
}
