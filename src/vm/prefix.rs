extern crate log;
extern crate pest;
use crate::lang::Rule;
use crate::vm::vm;

pub fn process_token(b: &mut vm::VM, p: &pest::iterators::Pair<Rule>, t: &String) {
    log::debug!("Received PREFIX token: {:#?}({})", p.as_rule(), t);
    let mut v = b.value().unwrap();
    v.prefix = t.to_string();
    match Some(&*v.prefix) {
        Some("$") => {
            log::debug!("Codeblock requested");
            b.state.codeblock_requested = true;
        }
        Some("~") => {
            log::debug!("GETALL from stack requested");
            b.state.get_all_requested = true;
        }
        Some("`") => {
            log::debug!("DEFINEPTR requested");
            b.state.defineptr_requested = true;
        }
        _  => {
            log::debug!("Prefix {} is not known", &v.prefix)
        }
    }
    b.add_value(v);
}
