extern crate log;
use rust_multistackvm::multistackvm::{VM};
use rust_dynamic::value::Value;
use easy_error::{Error, bail};

pub fn stdlib_conditional_curry(vm: &mut VM) -> Result<&mut VM, Error> {
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
    res = res.set("type", Value::from_string("curry"));
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
    let _cond_name = match cond_name_val.cast_string() {
        Ok(cond_name) => cond_name,
        Err(err) => bail!("CONTEXT.RUN: casting context name returns error: {}", err),
    };

    Ok(vm)
}
