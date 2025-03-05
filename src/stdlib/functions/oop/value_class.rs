extern crate log;
use crate::cmd;
use crate::stdlib::BUND;
use rust_dynamic::types::*;
use rust_dynamic::value::Value;
use rust_multistackvm::multistackvm::VM;
use crate::stdlib::{helpers, functions};
use easy_error::{Error, bail};

fn register_method_value_init(vm: &mut VM) -> Result<&mut VM, Error> {
    let mut value = match vm.stack.pull() {
        Some(value) => value,
        None => bail!("Stack is empty for method '.init' of object Value"),
    };
    if ! value.type_of() == OBJECT {
        bail!("Object is expected in the stack for '.init' method of object Value");
    }
    let data_value = match vm.stack.pull() {
        Some(data_value) => data_value,
        None => bail!("Object(Value) NO DATA #1"),
    };
    value = value.set(".data".to_string(), data_value);
    vm.stack.push(value);
    Ok(vm)
}


fn register_value(vm: &mut VM) -> Result<&mut VM, Error> {
    let _ = vm.register_method(".value_init".to_string(), register_method_value_init);
    let mut obj_class = Value::make_class();
    let mut super_class = Value::list();
    super_class = super_class.push(Value::from_string("Object"));
    super_class = super_class.push(Value::from_string("Iterable"));
    obj_class = obj_class.set(".class_name".to_string(), Value::from_string("Value"));
    obj_class = obj_class.set(".super".to_string(), super_class);
    obj_class = obj_class.set(".init".to_string(), Value::ptr(".value_init".to_string(), Vec::new()));
    vm.register_class("Value".to_string(), obj_class)
}

pub fn stdlib_object_value_wrap(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 2 {
        bail!("Stack is too shallow for inline UNWRAP");
    }
    let mut obj_val = match vm.stack.pull() {
        Some(obj_val) => if obj_val.type_of() == OBJECT {
            obj_val
        } else {
            bail!("UNWRAP NO OBJECT IN #1");
        },
        None => bail!("UNWRAP NO DATA IN #1"),
    };
    match obj_val.has_key_raw(".data") {
        Ok(is_key) => {
            if ! is_key {
                bail!("WRAP: this object is not devrived from Value class");
            }
        }
        Err(err) => bail!("WRAP: can not detect if OBJECT is wrappable: {}", err),

    }
    let data_val = match vm.stack.pull() {
        Some(data_val) => data_val,
        None => bail!("WRAP NO DATA IN #1"),
    };
    obj_val = obj_val.set(".data", data_val);
    vm.stack.push(obj_val);
    Ok(vm)
}

pub fn stdlib_object_value_unwrap(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline UNWRAP");
    }
    let obj_val = match vm.stack.pull() {
        Some(obj_val) => if obj_val.type_of() == OBJECT {
            obj_val
        } else {
            bail!("UNWRAP NO OBJECT IN #1");
        },
        None => bail!("UNWRAP NO DATA IN #1"),
    };
    match obj_val.get(".data") {
        Ok(value) => vm.stack.push(value),
        Err(err) => bail!("UNWRAP found no wrapped VALUE: {}", err),
    };
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
    //
    // Register Value
    //
    match register_value(&mut bc.vm) {
        Ok(_) => {},
        Err(err) => {
            log::error!("Error registeing Value base class: {}", err);
            bc.vm.stack.push(Value::from_int(10));
            let _ = functions::bund::bund_exit::stdlib_bund_exit_inline(&mut bc.vm);
        }
    }
    let _ = bc.vm.register_inline("wrap".to_string(), stdlib_object_value_wrap);
    let _ = bc.vm.register_inline("unwrap".to_string(), stdlib_object_value_unwrap);
    drop(bc);
}
