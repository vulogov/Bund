extern crate log;
extern crate pest;
use crate::lang::Rule;
use crate::vm::vm;
use lexical_core;

pub fn process_token(b: &mut vm::VM, p: &pest::iterators::Pair<Rule>, t: &String) {
    log::debug!("Received FLOAT token: {:#?}({})", p.as_rule(), t);
    let num: Result<f64, lexical_core::Error> = lexical_core::parse(t.as_bytes());
    match num {
        Ok(val) => {
            let mut v = b.value().unwrap();
            v.float(val);
            b.add_value(v);
        }
        Err(err) => {
            log::error!("Error parsing FLOAT token: {:?}", err);
        }
    }
}
