extern crate log;
use std::collections;
use crate::twostack::value;
use crate::vm::vm;
use crate::vm::bundfunction;

fn stdlib_bund_print_print(_b: &mut vm::VM, _q: &mut collections::VecDeque<value::Value>) {

}

pub fn init_stdlib(b: &mut vm::VM) {
    log::debug!("BUND print stdlib init reached");
    b.add_function("print", bundfunction::BundFunction::new("print", 0, stdlib_bund_print_print));
}
