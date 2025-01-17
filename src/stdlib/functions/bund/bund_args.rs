extern crate log;
use crate::cmd;
use crate::stdlib::BUND;
use crate::stdlib::helpers;
use argmap;
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

#[time_graph::instrument]
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

#[time_graph::instrument]
pub fn stdlib_bund_args_parse(vm: &mut VM) -> Result<&mut VM, Error> {
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
    let mut arg: Vec<String> = Vec::new();
    let raw_args = match res.cast_list() {
        Ok(raw_args) => raw_args,
        Err(err) => bail!("Unable to cast list of ARGS: {}", err),
    };
    for a in raw_args {
        let an_arg = match a.cast_string() {
            Ok(an_arg) => an_arg,
            Err(err) => bail!("Unable to argument of ARGS: {}", err),
        };
        arg.push(an_arg);
    }
    let (args,argv) = argmap::parse(arg.iter());
    let mut out = Value::dict();
    let mut args_list = Value::list();
    for i in args {
        args_list = args_list.push(Value::from_string(i));
    }
    for (k,v) in argv {
        let mut val_list = Value::list();
        for v_item in v {
            val_list = val_list.push(Value::from_string(v_item));
        }
        out = out.set(k, val_list);
    }
    out = out.set("args", args_list);
    vm.stack.push(out);
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
    let _ = bc.vm.register_inline("args.parse".to_string(), stdlib_bund_args_parse);
    drop(bc);
}
