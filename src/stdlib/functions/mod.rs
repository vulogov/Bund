extern crate log;

pub mod bund;
pub mod input;

pub fn init_stdlib() {
    bund::init_stdlib();
    input::init_stdlib();
}
