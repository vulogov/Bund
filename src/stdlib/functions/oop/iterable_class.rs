extern crate log;
use crate::cmd;
use crate::stdlib::BUND;
use rust_dynamic::types::*;
use rust_dynamic::value::Value;
use rust_multistackvm::multistackvm::VM;
use crate::stdlib::{helpers, functions};
use easy_error::{Error, bail};

fn register_method_value_next(vm: &mut VM) -> Result<&mut VM, Error> {
    let mut value = match vm.stack.pull() {
        Some(value) => value,
        None => bail!("Stack is empty for method '.init' of object Value"),
    };
    if ! value.type_of() == OBJECT {
        bail!("Object is expected in the stack for '.init' method of object Value");
    }
    vm.stack.push(value.clone());
    match value.next() {
        Some(n_value) => vm.stack.push(n_value),
        None => vm.stack.push(Value::none()),
    };
    Ok(vm)
}



fn register_iterable(vm: &mut VM) -> Result<&mut VM, Error> {
    let _ = vm.register_method(".iteranble_next".to_string(), register_method_value_next);
    let mut obj_class = Value::make_class();
    let super_class = Value::list();
    obj_class = obj_class.set(".class_name".to_string(), Value::from_string("Value"));
    obj_class = obj_class.set(".super".to_string(), super_class);
    obj_class = obj_class.set(".next".to_string(), Value::ptr(".iterable_next".to_string(), Vec::new()));
    vm.register_class("Iterable".to_string(), obj_class)
}



pub fn init_stdlib(cli: &cmd::Cli) {
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };
    //
    // Register Value
    //
    match register_iterable(&mut bc.vm) {
        Ok(_) => {},
        Err(err) => {
            log::error!("Error registeing Iterable base class: {}", err);
            bc.vm.stack.push(Value::from_int(10));
            let _ = functions::bund::bund_exit::stdlib_bund_exit_inline(&mut bc.vm);
        }
    }
    drop(bc);
}
