extern crate log;
use crate::stdlib::BUND;
use rust_multistackvm::multistackvm::{VM};
use rust_dynamic::value::Value;
use crate::cmd;
use crate::stdlib::helpers;
use easy_error::{Error};

use uuid::Uuid;
use ulid::Ulid;

#[time_graph::instrument]
pub fn stdlib_uuid_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    vm.stack.push(Value::from_string(Uuid::new_v4().to_string()));
    Ok(vm)
}

#[time_graph::instrument]
pub fn stdlib_ulid_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    let ulid = Ulid::new();
    vm.stack.push(Value::from_string(ulid.to_string()));
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
    let _ = bc.vm.register_inline("id.uuid".to_string(), stdlib_uuid_inline);
    let _ = bc.vm.register_inline("id.ulid".to_string(), stdlib_ulid_inline);
    drop(bc);
}
