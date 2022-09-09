extern crate log;
use crate::vm::vm;

pub mod datatype;
pub mod print;

pub fn init_stdlib(b: &mut vm::VM) {
    log::debug!("Root stdlib init reached");
    datatype::init_stdlib(b);
    print::init_stdlib(b);

}
