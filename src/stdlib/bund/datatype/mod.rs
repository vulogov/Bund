extern crate log;
use crate::vm::vm;

pub fn init_stdlib(_b: &mut vm::VM) {
    log::debug!("BUND datatypes stdlib init reached");
}
