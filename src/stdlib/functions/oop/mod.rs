extern crate log;
use crate::cmd;

pub mod base_classes;
pub mod value_class;
pub mod display_class;
pub mod iterable_class;
pub mod int_class;
pub mod float_class;
pub mod bool_class;
pub mod object_loop;

pub fn init_stdlib(cli: &cmd::Cli) {
    base_classes::init_stdlib(cli);
    value_class::init_stdlib(cli);
    display_class::init_stdlib(cli);
    iterable_class::init_stdlib(cli);
    int_class::init_stdlib(cli);
    float_class::init_stdlib(cli);
    bool_class::init_stdlib(cli);
    object_loop::init_stdlib(cli);
}
