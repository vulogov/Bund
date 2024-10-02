extern crate log;
use crate::cmd;
use bundcore::bundcore::Bund;
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref BUND: Mutex<Bund> = {
        let e: Mutex<Bund> = Mutex::new(Bund::new());
        e
    };
}

pub mod helpers;
pub mod functions;

pub fn init_stdlib(cli: &cmd::Cli) {
    log::debug!("Initialize BUND core");
    let bc = BUND.lock().unwrap();
    drop(bc);
    log::debug!("Running STDLIB init");
    functions::init_stdlib(cli);
}
