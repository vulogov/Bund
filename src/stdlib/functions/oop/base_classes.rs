extern crate log;
use crate::cmd;
use crate::stdlib::BUND;
use rust_dynamic::types::*;
use rust_dynamic::value::Value;
use rust_multistackvm::multistackvm::VM;
use crate::stdlib::{helpers, functions};
use easy_error::{Error, bail};

fn register_method_id(vm: &mut VM) -> Result<&mut VM, Error> {
    let value = match vm.stack.peek() {
        Some(value) => value,
        None => bail!("Stack is empty for method '.id'"),
    };
    vm.stack.push(Value::from_string(value.id));
    Ok(vm)
}

fn register_method_timestamp(vm: &mut VM) -> Result<&mut VM, Error> {
    let value = match vm.stack.peek() {
        Some(value) => value,
        None => bail!("Stack is empty for method '.timestamp'"),
    };
    vm.stack.push(Value::from_float(value.stamp));
    Ok(vm)
}

fn register_method_to_str(vm: &mut VM) -> Result<&mut VM, Error> {
    let value = match vm.stack.peek() {
        Some(value) => value,
        None => bail!("Stack is empty for method '.str'"),
    };
    match value.type_of() {
        OBJECT => {
            let cl_name_val = match value.get(".class_name") {
                Ok(class_name) => class_name,
                Err(err) => bail!("'.str' error getting class name: {}", err),
            };
            let cl_name = match cl_name_val.cast_string() {
                Ok(cl_name) => cl_name,
                Err(err) => bail!("'.str' error casting class name: {}", err),
            };
            match value.get(".data") {
                Ok(data_val) => {
                    match data_val.conv(STRING) {
                        Ok(str_value) => {
                            vm.stack.push(str_value);
                        }
                        Err(err) => {
                            bail!("'.str' returns: {}", err);
                        }
                    }
                }
                Err(_) => {
                    vm.stack.push(Value::from_string(format!("Object({})", &cl_name)));
                }
            };
        }
        _ => {
            match value.conv(STRING) {
                Ok(str_value) => {
                    vm.stack.push(str_value);
                }
                Err(err) => {
                    bail!("'.str' returns: {}", err);
                }
            }
        }
    }
    Ok(vm)
}

fn register_method_print(vm: &mut VM) -> Result<&mut VM, Error> {
    match register_method_to_str(vm) {
        Ok(_) => {},
        Err(err) => bail!("{}", err),
    };
    rust_multistackvm::stdlib::print::stdlib_print_inline(vm)
}

fn register_method_println(vm: &mut VM) -> Result<&mut VM, Error> {
    match register_method_to_str(vm) {
        Ok(_) => {},
        Err(err) => bail!("{}", err),
    };
    rust_multistackvm::stdlib::print::stdlib_println_inline(vm)
}

fn register_object(vm: &mut VM) -> Result<&mut VM, Error> {
    let _ = vm.register_method(".id".to_string(), register_method_id);
    let _ = vm.register_method(".timestamp".to_string(), register_method_timestamp);
    let mut obj_class = Value::make_class();
    let mut super_class = Value::list();
    super_class = super_class.push(Value::from_string("Printable"));
    obj_class = obj_class.set(".class_name".to_string(), Value::from_string("Object"));
    obj_class = obj_class.set(".super".to_string(), super_class);
    obj_class = obj_class.set(".id".to_string(), Value::ptr(".id".to_string(), Vec::new()));
    obj_class = obj_class.set(".timestamp".to_string(), Value::ptr(".timestamp".to_string(), Vec::new()));
    vm.register_class("Object".to_string(), obj_class)
}

fn register_printable(vm: &mut VM) -> Result<&mut VM, Error> {
    let _ = vm.register_method(".str".to_string(), register_method_to_str);
    let _ = vm.register_method(".print".to_string(), register_method_print);
    let _ = vm.register_method(".println".to_string(), register_method_println);
    let mut obj_class = Value::make_class();
    let mut super_class = Value::list();
    super_class = super_class.push(Value::from_string("Display"));
    obj_class = obj_class.set(".class_name".to_string(), Value::from_string("Printable"));
    obj_class = obj_class.set(".super".to_string(), super_class);
    obj_class = obj_class.set("str".to_string(), Value::ptr(".str".to_string(), Vec::new()));
    obj_class = obj_class.set("print".to_string(), Value::ptr(".print".to_string(), Vec::new()));
    obj_class = obj_class.set("println".to_string(), Value::ptr(".println".to_string(), Vec::new()));
    vm.register_class("Printable".to_string(), obj_class)
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
    // Register base hierarchy
    //
    match register_object(&mut bc.vm) {
        Ok(_) => {},
        Err(err) => {
            log::error!("Error registeing Object base class: {}", err);
            bc.vm.stack.push(Value::from_int(10));
            let _ = functions::bund::bund_exit::stdlib_bund_exit_inline(&mut bc.vm);
        }
    }
    match register_printable(&mut bc.vm) {
        Ok(_) => {},
        Err(err) => {
            log::error!("Error registeing Printable base class: {}", err);
            bc.vm.stack.push(Value::from_int(10));
            let _ = functions::bund::bund_exit::stdlib_bund_exit_inline(&mut bc.vm);
        }
    }

    drop(bc);
}
