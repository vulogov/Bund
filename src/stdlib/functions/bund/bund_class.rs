extern crate log;
use crate::cmd;
use crate::stdlib::BUND;
use crate::stdlib::helpers;
use rust_dynamic::value::Value;
use rust_dynamic::types::*;
use rust_multistackvm::multistackvm::{VM};
use easy_error::{Error, bail};

#[time_graph::instrument]
pub fn stdlib_bund_is_class(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for ?CLASS");
    }
    let cl_name_value = match vm.stack.pull() {
        Some(cl_name_value) => cl_name_value,
        None => {
            bail!("?CLASS returns NO DATA #1");
        }
    };
    let cl_name = match cl_name_value.cast_string() {
        Ok(cl_name) => cl_name,
        Err(err) => {
            bail!("?CLASS casting string returns: {}", err);
        }
    };
    let is_class = vm.is_class(cl_name.clone());
    log::debug!("?CLASS: checking if {} is a registered class - {}", &cl_name, is_class);
    let res = Value::from_bool(is_class);
    vm.stack.push(res);
    Ok(vm)
}

#[time_graph::instrument]
pub fn stdlib_bund_is_object(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for ?OBJECT");
    }
    let ob_value = match vm.stack.peek() {
        Some(ob_value) => ob_value,
        None => {
            bail!("?OBJECT returns NO DATA #1");
        }
    };
    match ob_value.type_of() {
        OBJECT => vm.stack.push(Value::from_bool(true)),
        _ =>  vm.stack.push(Value::from_bool(false)),
    };
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
    let _ = bc.vm.register_inline("?class".to_string(), stdlib_bund_is_class);
    let _ = bc.vm.register_inline("?object".to_string(), stdlib_bund_is_object);
    drop(bc);
}
