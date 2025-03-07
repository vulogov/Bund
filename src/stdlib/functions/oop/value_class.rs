extern crate log;
use crate::cmd;
use crate::stdlib::BUND;
use rust_dynamic::types::*;
use rust_dynamic::value::Value;
use rust_multistackvm::multistackvm::VM;
use crate::stdlib::{helpers, functions};
use easy_error::{Error, bail};

pub fn locate_value_in_object(name: String, value: Value) -> Option<Value> {
    match value.get(name.clone()) {
        Ok(value) => return Some(value),
        Err(_) => {}
    }
    let super_list = match value.get(".super") {
        Ok(super_list) => super_list,
        Err(_) => return None,
    };
    if super_list.type_of() != LIST {
        return None;
    }
    for s in super_list {
        match locate_value_in_object(name.clone(), s) {
            Some(v) => return Some(v),
            None => continue,
        }
    }
    return None;
}

pub fn set_value_in_object(name: String, mut value: Value, n_value: Value) -> Value {
    match value.get(name.clone()) {
        Ok(_) => {
            value = value.set(name, n_value);
            return value;
        }
        Err(_) => {}
    }
    let super_list = match value.get(".super") {
        Ok(super_list) => super_list,
        Err(_) => return value,
    };
    if super_list.type_of() != LIST {
        return value;
    }
    let mut new_super = Value::list();
    for s in super_list {
        new_super = new_super.push(set_value_in_object(name.clone(), s, n_value.clone()))
    }
    value = value.set(".super".to_string(), new_super);
    return value;
}

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
    let obj_val = match vm.stack.pull() {
        Some(obj_val) => if obj_val.type_of() == OBJECT {
            obj_val
        } else {
            bail!("UNWRAP NO OBJECT IN #1");
        },
        None => bail!("UNWRAP NO DATA IN #1"),
    };
    match locate_value_in_object(".data".to_string(), obj_val.clone()) {
        Some(_) => {}
        None => bail!("WRAP: can not detect if OBJECT is wrappable"),

    }
    let data_val = match vm.stack.pull() {
        Some(data_val) => data_val,
        None => bail!("WRAP NO DATA IN #1"),
    };
    vm.stack.push(set_value_in_object(".data".to_string(), obj_val.clone(), data_val));
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
            bail!("UNWRAP: NO OBJECT IN #1");
        },
        None => bail!("UNWRAP: NO DATA IN #1"),
    };
    match locate_value_in_object(".data".to_string(), obj_val) {
        Some(value) => vm.stack.push(value),
        None => bail!("UNWRAP: found no wrapped VALUE"),
    };
    Ok(vm)
}

pub fn stdlib_object_value_is(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline IS");
    }
    let obj_val = match vm.stack.pull() {
        Some(obj_val) => if obj_val.type_of() == OBJECT {
            obj_val
        } else {
            bail!("IS: NO OBJECT IN #1");
        },
        None => bail!("IS: NO DATA IN #1"),
    };
    match locate_value_in_object(".data".to_string(), obj_val.clone()) {
        Some(value) => {
            vm.stack.push(obj_val);
            vm.stack.push(value);
        }
        None => bail!("IS: found no wrapped VALUE"),
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
    let _ = bc.vm.register_inline("is".to_string(), stdlib_object_value_is);
    drop(bc);
}
