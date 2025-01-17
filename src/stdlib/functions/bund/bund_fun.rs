extern crate log;
use crate::cmd;
use crate::stdlib::BUND;
use crate::stdlib::helpers;
use rust_dynamic::value::Value;
use rust_multistackvm::multistackvm::{VM};
use easy_error::{Error, bail};


pub fn stdlib_bund_is_alias(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for ?ALIAS");
    }
    let fn_name_value = match vm.stack.pull() {
        Some(file_name_value) => file_name_value,
        None => {
            bail!("?ALIAS returns NO DATA #1");
        }
    };
    let fn_name = match fn_name_value.cast_string() {
        Ok(file_name) => file_name,
        Err(err) => {
            bail!("?ALIAS casting string returns: {}", err);
        }
    };
    let res = Value::from_bool(vm.is_alias(fn_name));
    vm.stack.push(res);
    Ok(vm)
}

pub fn stdlib_bund_is_lambda(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for ?LAMBDA");
    }
    let fn_name_value = match vm.stack.pull() {
        Some(file_name_value) => file_name_value,
        None => {
            bail!("?LAMBDA returns NO DATA #1");
        }
    };
    let fn_name = match fn_name_value.cast_string() {
        Ok(file_name) => file_name,
        Err(err) => {
            bail!("?LAMBDA casting string returns: {}", err);
        }
    };
    let res = Value::from_bool(vm.is_lambda(fn_name));
    vm.stack.push(res);
    Ok(vm)
}

pub fn stdlib_bund_get_alias(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for ?ALIAS");
    }
    let fn_name_value = match vm.stack.pull() {
        Some(file_name_value) => file_name_value,
        None => {
            bail!("?ALIAS.GET returns NO DATA #1");
        }
    };
    let fn_name = match fn_name_value.cast_string() {
        Ok(file_name) => file_name,
        Err(err) => {
            bail!("?ALIAS.GET casting string returns: {}", err);
        }
    };
    match vm.get_alias(fn_name) {
        Ok(fn_alias) => {
            vm.stack.push(Value::from_string(fn_alias));
        }
        Err(err) => {
            bail!("ALIAS.GET returned: {}", err);
        }
    }
    Ok(vm)
}

pub fn stdlib_bund_get_lambda(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for LAMBDA.GET");
    }
    let fn_name_value = match vm.stack.pull() {
        Some(file_name_value) => file_name_value,
        None => {
            bail!("LAMBDA.GET returns NO DATA #1");
        }
    };
    let fn_name = match fn_name_value.cast_string() {
        Ok(file_name) => file_name,
        Err(err) => {
            bail!("LAMBDA.GET casting string returns: {}", err);
        }
    };
    match vm.get_lambda(fn_name) {
        Ok(fn_value) => {
            vm.stack.push(fn_value);
        }
        Err(err) => {
            bail!("LAMBDA.GET returned: {}", err);
        }
    }
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
    let _ = bc.vm.register_inline("?alias".to_string(), stdlib_bund_is_alias);
    let _ = bc.vm.register_inline("?lambda".to_string(), stdlib_bund_is_lambda);
    let _ = bc.vm.register_inline("alias=".to_string(), stdlib_bund_get_alias);
    let _ = bc.vm.register_inline("lambda=".to_string(), stdlib_bund_get_lambda);
    drop(bc);
}
