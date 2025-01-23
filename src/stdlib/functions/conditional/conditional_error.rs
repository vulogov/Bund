extern crate log;
use rust_multistackvm::multistackvm::{VM};
use rust_dynamic::value::Value;
use crate::cmd::CLI;
use crate::stdlib::helpers;
use easy_error::{Error, bail};

pub fn stdlib_conditional_error(vm: &mut VM) -> Result<&mut VM, Error> {
    let mut res: Value = Value::conditional();
    res = res.set("type", Value::from_string("error"));
    vm.stack.push(res);
    Ok(vm)
}

pub fn conditional_run(vm: &mut VM, value: Value) -> Result<&mut VM, Error> {
    log::debug!("Running ERROR conditional");
    let msg = match value.get("context") {
        Ok(msg) => msg,
        Err(_) => Value::from_string(""),
    };
    let associated_lambda = match value.get("associated") {
        Ok(associated_lambda) => associated_lambda,
        Err(_) => Value::lambda(),
    };
    let cli = CLI.lock().unwrap();
    helpers::print_error::print_error_from_str_with_vm(vm, msg.to_string(), &cli);
    drop(cli);
    match vm.lambda_eval(associated_lambda) {
        Ok(res) => return Ok(res),
        Err(err) => bail!("ERROR ASSOCIATED lambda returns: {}", err),
    };
}
