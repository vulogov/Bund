extern crate log;
use rust_multistackvm::multistackvm::{VM};
use rust_dynamic::value::Value;
use easy_error::{Error, bail};

pub fn stdlib_conditional_ctx(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for context");
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
    res = res.set("type", Value::from_string("context"));
    res = res.set("name", Value::from_string(name));
    vm.stack.push(res);
    Ok(vm)
}

pub fn conditional_run(vm: &mut VM, value: Value) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for CONTEXT.RUN");
    }
    let cond_name_val = match value.get("name") {
        Ok(cond_name_val) => cond_name_val,
        Err(err) => bail!("CONTEXT.RUN: getting context name returns error: {}", err),
    };
    let cond_name = match cond_name_val.cast_string() {
        Ok(cond_name) => cond_name,
        Err(err) => bail!("CONTEXT.RUN: casting context name returns error: {}", err),
    };
    let prev_stack_name = match vm.stack.current_stack_name() {
        Some(prev_stack_name) => prev_stack_name,
        None => bail!("CONTEXT.RUN: No current context discovered"),
    };
    let name_val = match vm.stack.pull() {
        Some(name_val) => name_val,
        None => bail!("CONTEXT.RUN: No context name discovered on the stack"),
    };
    let name = match name_val.cast_string() {
        Ok(name) => name,
        Err(err) => bail!("CONTEXT.RUN: Error name casting: {}", err),
    };
    let pre_lambda = match value.get(format!("{}.pre", &name)) {
        Ok(pre_lambda) => pre_lambda,
        Err(_) => Value::lambda(),
    };
    let post_lambda = match value.get(format!("{}.post", &name)) {
        Ok(post_lambda) => post_lambda,
        Err(_) => Value::lambda(),
    };
    let run_lambda = match value.get(format!("{}", &name)) {
        Ok(run_lambda) => run_lambda,
        Err(_) => Value::lambda(),
    };
    let _ = vm.stack.to_stack(cond_name);
    match vm.lambda_eval(pre_lambda) {
        Ok(_) => {},
        Err(err) => bail!("CONTEXT PRE lambda returns: {}", err),
    };
    match vm.lambda_eval(run_lambda) {
        Ok(_) => {},
        Err(err) => bail!("CONTEXT RUN lambda returns: {}", err),
    };
    match vm.lambda_eval(post_lambda) {
        Ok(_) => {},
        Err(err) => bail!("CONTEXT POST lambda returns: {}", err),
    };
    let _ = vm.stack.to_stack(prev_stack_name);
    Ok(vm)
}
