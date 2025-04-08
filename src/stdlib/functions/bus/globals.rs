extern crate log;
use crate::stdlib::BUND;
use rust_multistackvm::multistackvm::{VM};
use rust_dynamic::value::Value;
use crate::cmd;
use crate::stdlib::helpers;
use easy_error::{Error, bail};


#[time_graph::instrument]
pub fn stdlib_global_set(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 2 {
        bail!("Stack is too shallow for GLOBAL");
    }
    let value = match vm.stack.pull() {
        Some(value) => value,
        None => bail!("GLOBAL: NO DATA #1"),
    };
    let name = match vm.stack.pull() {
        Some(name_val) => match name_val.cast_string() {
            Ok(name) => name,
            Err(err) => bail!("GLOBAL: ERROR CASTING NAME: {}", err),
        },
        None => bail!("GLOBAL: NO DATA #2"),
    };
    let key = match helpers::zenoh::conf::get_globals_path(name.clone()) {
        Ok(key) => key,
        Err(err) => bail!("GLOBAL: error setting KEY: {}", err),
    };
    let receiving = match helpers::zenoh::conf::get_receiving_path(name.clone()) {
        Ok(key) => key,
        Err(err) => bail!("GLOBAL: error setting KEY: {}", err),
    };
    let from_addr = Value::from_string(receiving.clone());
    let to_addr = Value::from_string(&key);
    let payload = Value::message(from_addr, to_addr, value);
    match helpers::zenoh::putget::zenoh_put_internal(key.clone(), payload) {
        Ok(_) => {},
        Err(err) => bail!("GLOBAL returns: {}", err),
    };
    log::debug!("GLOBAL set for key {}", &key);
    Ok(vm)
}

#[time_graph::instrument]
pub fn stdlib_global_get(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for GLOBAL");
    }
    let name = match vm.stack.pull() {
        Some(name_val) => match name_val.cast_string() {
            Ok(name) => name,
            Err(err) => bail!("GLOBAL: ERROR CASTING NAME: {}", err),
        },
        None => bail!("GLOBAL: NO DATA #1"),
    };
    let key = match helpers::zenoh::conf::get_globals_path(name.clone()) {
        Ok(key) => key,
        Err(err) => bail!("GLOBAL*: error setting KEY: {}", err),
    };
    let res = match helpers::zenoh::putget::zenoh_get_internal(key.clone()) {
        Ok(res) => res,
        Err(err) => bail!("GLOBAL* returns: {}", err),
    };
    for message in res {
        let payload = match message.at(2) {
            Some(payload) => payload,
            None => bail!("GLOBAL* error getting payload"),
        };
        for v in payload {
            vm.stack.push(v);
        }
    }
    Ok(vm)
}

#[time_graph::instrument]
pub fn stdlib_global_check(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for GLOBAL");
    }
    let name = match vm.stack.pull() {
        Some(name_val) => match name_val.cast_string() {
            Ok(name) => name,
            Err(err) => bail!("GLOBAL: ERROR CASTING NAME: {}", err),
        },
        None => bail!("GLOBAL: NO DATA #1"),
    };
    let key = match helpers::zenoh::conf::get_globals_path(name.clone()) {
        Ok(key) => key,
        Err(err) => bail!("GLOBAL*: error setting KEY: {}", err),
    };
    let res = match helpers::zenoh::putget::zenoh_has_get_internal(key.clone()) {
        Ok(res) => res,
        Err(err) => bail!("GLOBAL* returns: {}", err),
    };
    vm.stack.push(Value::from_bool(res));
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
    let _ = bc.vm.register_inline("global".to_string(), stdlib_global_set);
    let _ = bc.vm.register_inline("global*".to_string(), stdlib_global_get);
    let _ = bc.vm.register_inline("?global".to_string(), stdlib_global_check);
    drop(bc);
}
