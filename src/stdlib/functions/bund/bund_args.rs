
extern crate log;
use crate::cmd;
use crate::stdlib::BUND;
use crate::stdlib::helpers;
use rust_dynamic::value::Value;
use rust_multistackvm::multistackvm::{VM};
use std::sync::Mutex;
use easy_error::{Error, bail};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref ARGS: Mutex<Value> = {
        let e: Mutex<Value> = Mutex::new(Value::list());
        e
    };
}

pub fn stdlib_bund_args(vm: &mut VM) -> Result<&mut VM, Error> {
    let args = match ARGS.lock() {
        Ok(args) => args,
        Err(err) => {
            bail!("Can not access ARGS: {}", err);
        }
    };
    let res = match args.dup() {
        Ok(res) => res,
        Err(err) => {
            bail!("Can not duplicate ARGS: {}", err);
        }
    };
    drop(args);
    vm.stack.push(res);
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
    let args = match ARGS.lock() {
        Ok(args) => args,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };
    drop(args);
    let _ = bc.vm.register_inline("args".to_string(), stdlib_bund_args);
    drop(bc);
}
