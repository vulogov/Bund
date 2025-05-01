extern crate log;
use crate::stdlib::BUND;
use rust_multistackvm::multistackvm::{VM};
use crate::cmd;
use std::time::Duration;
use spin_sleep;
use crate::stdlib::helpers;
use easy_error::{Error, bail};


pub fn stdlib_sleep_seconds(vm: &mut VM) -> Result<&mut VM, Error> {
    let n = match vm.stack.pull() {
        Some(n) => match n.cast_int() {
            Ok(n) => n,
            Err(err) => bail!("SLEEP::SECONDS error casting seconds: {}", err),
        },
        None => bail!("SLEEP::SECONDS: NO DATA #1"),
    };
    spin_sleep::sleep(Duration::new(n as u64,0));
    Ok(vm)
}


pub fn init_stdlib(cli: &cmd::Cli) {
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };
    let _ = bc.vm.register_inline("sleep.seconds".to_string(), stdlib_sleep_seconds);

    drop(bc);
}
