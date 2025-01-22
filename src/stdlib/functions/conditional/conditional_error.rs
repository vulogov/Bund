extern crate log;
use rust_multistackvm::multistackvm::{VM};
use rust_dynamic::value::Value;
use easy_error::{Error};

pub fn stdlib_conditional_error(vm: &mut VM) -> Result<&mut VM, Error> {
    let mut res: Value = Value::conditional();
    res = res.set("type", Value::from_string("error"));
    vm.stack.push(res);
    Ok(vm)
}
