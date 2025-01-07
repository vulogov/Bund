
extern crate log;
use crate::cmd;
use crate::stdlib::BUND;
use crate::stdlib::helpers;
use rust_multistackvm::multistackvm::{VM};
use easy_error::{Error};
use std::process;

pub fn stdlib_bund_exit_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() == 0 {
        log::debug!("BUND is exiting error code 0. Stack is empty");
        process::exit(0);
    }
    let err_code_val = match vm.stack.pull() {
        Some(err_code_val) => err_code_val,
        None => {
            process::exit(0);
        },
    };
    let err_code = match err_code_val.cast_int() {
        Ok(err_code) => err_code,
        Err(err) => {
            log::error!("Error in casting error code for exit: {}", err);
            0 as i64
        },
    };
    log::debug!("BUND is exiting with code {}", err_code);
    process::exit(err_code as i32);
}

pub fn init_stdlib(cli: &cmd::Cli) {
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };
    let _ = bc.vm.register_inline("bund.exit".to_string(), stdlib_bund_exit_inline);
    drop(bc);
}
