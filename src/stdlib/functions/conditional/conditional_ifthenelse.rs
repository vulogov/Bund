extern crate log;
use rust_multistackvm::multistackvm::{VM};

use rust_dynamic::value::Value;
use easy_error::{Error, bail};

pub fn stdlib_conditional_ifthenelse(vm: &mut VM) -> Result<&mut VM, Error> {
    let mut res: Value = Value::conditional();
    res = res.set("type", Value::from_string("ifthenelse"));
    vm.stack.push(res);
    Ok(vm)
}

pub fn conditional_run(vm: &mut VM, value: Value) -> Result<&mut VM, Error> {
    let if_lambda = match value.get("if") {
        Ok(if_lambda) => if_lambda,
        Err(_) => Value::lambda(),
    };
    let then_lambda = match value.get("then") {
        Ok(then_lambda) => then_lambda,
        Err(_) => Value::lambda(),
    };
    let else_lambda = match value.get("else") {
        Ok(else_lambda) => else_lambda,
        Err(_) => Value::lambda(),
    };
    match vm.lambda_eval(if_lambda) {
        Ok(_) => {},
        Err(err) => bail!("IFTHENELSE IF lambda returns: {}", err),
    };
    let cond_value = match vm.stack.pull() {
        Some(cond_value) => cond_value,
        None => bail!("IFTHENELSE conditional require condition on the stack"),
    };
    let cond = match cond_value.cast_bool() {
        Ok(cond) => cond,
        Err(err) => bail!("IFTHENELSE error casting conditional: {}", err),
    };
    if cond {
        match vm.lambda_eval(then_lambda) {
            Ok(res) => return Ok(res),
            Err(err) => bail!("IFTHENELSE THEN lambda returns: {}", err),
        };
    } else {
        match vm.lambda_eval(else_lambda) {
            Ok(res) => return Ok(res),
            Err(err) => bail!("IFTHENELSE ELSE lambda returns: {}", err),
        };
    }
}
