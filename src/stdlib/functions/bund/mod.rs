extern crate log;

pub mod bund_eval;

pub fn init_stdlib() {
    log::debug!("Initializing BUND: module");
    bund_eval::init_stdlib();
}
