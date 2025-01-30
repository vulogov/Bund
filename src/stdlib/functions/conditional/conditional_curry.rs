extern crate log;
use rust_multistackvm::multistackvm::{VM};
use rust_dynamic::value::Value;
use easy_error::{Error, bail};

pub fn stdlib_conditional_curry(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for curry");
    }
    let name_val = match vm.stack.pull() {
        Some(name_val) => name_val,
        None => bail!("CONTEXT: No context name discovered on the stack"),
    };
    let name = match name_val.cast_string() {
        Ok(name) => name,
        Err(err) => bail!("CONTEXT: Error name casting: {}", err),
    };
    let mut res: Value = Value::conditional();
    res = res.set("type", Value::from_string("curry"));
    res = res.set("name", Value::from_string(name));
    vm.stack.push(res);
    Ok(vm)
}

pub fn conditional_run(vm: &mut VM, value: Value) -> Result<&mut VM, Error> {
    let cond_name_val = match value.get("name") {
        Ok(cond_name_val) => cond_name_val,
        Err(err) => bail!("CONTEXT.RUN: getting context name returns error: {}", err),
    };
    let cond_name = match cond_name_val.cast_string() {
        Ok(cond_name) => cond_name,
        Err(err) => bail!("CONTEXT.RUN: casting curry name returns error: {}", err),
    };
    let data_val = match value.get("data") {
        Ok(data_val) => data_val,
        Err(_) => Value::list(),
    };
    let data = match data_val.cast_list() {
        Ok(data) => data,
        Err(err) => bail!("CONTEXT.RUN: casting curry data returns error: {}", err),
    };
    let lambda_val = match value.get("lambda") {
        Ok(lambda_val) => lambda_val,
        Err(_) => Value::lambda(),
    };
    let mut res_lambda = Value::lambda();
    for i in data.into_iter().rev() {
        res_lambda = res_lambda.push(i.clone());
    }
    res_lambda = res_lambda.push(lambda_val);
    res_lambda = res_lambda.push(Value::call("!".to_string(), Vec::new()));
    let _ = vm.register_lambda(cond_name, res_lambda);
    Ok(vm)
}
