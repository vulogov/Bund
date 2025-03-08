extern crate log;
use crate::cmd;
use crate::stdlib::BUND;
use rust_dynamic::types::*;
use rust_dynamic::value::Value;
use rust_multistackvm::multistackvm::VM;
use crate::stdlib::{helpers, functions};
use crate::stdlib::functions::oop;
use easy_error::{Error, bail};

fn register_method_list_init(vm: &mut VM) -> Result<&mut VM, Error> {
    let value_object = match vm.stack.pull() {
        Some(value_object) => value_object,
        None => bail!("List: stack is empty"),
    };
    match oop::value_class::locate_value_in_object(".data".to_string(), value_object.clone()) {
        Some(data_object) => {
            match data_object.conv(LIST) {
                Ok(new_data_object) => {
                    let new_value_object = oop::value_class::set_value_in_object(".data".to_string(), value_object, new_data_object);
                    vm.stack.push(new_value_object);
                }
                Err(err) => bail!("List: error converting to BOOL: {}", err),
            }
        }
        None => bail!("List: NO WRAPPED DATA WAS FOUND"),
    }
    Ok(vm)
}

fn register_method_list_push(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 2 {
        bail!("Stack is too shallow for method 'List::push'");
    }
    let value_object = match vm.stack.pull() {
        Some(value_object) => value_object,
        None => bail!("List: NO DATA #1"),
    };
    let new_object = match vm.stack.pull() {
        Some(value_object) => value_object,
        None => bail!("List: NO DATA #2"),
    };
    match oop::value_class::locate_value_in_object(".data".to_string(), value_object.clone()) {
        Some(mut data_object) => {
            if data_object.type_of() == LIST {
                data_object = data_object.push(new_object);
            } else {
                bail!("List::push data object is not LIST");
            }
            let new_value_object = oop::value_class::set_value_in_object(".data".to_string(), value_object, data_object);
            vm.stack.push(new_value_object);
        }
        None => bail!("List: NO WRAPPED DATA WAS FOUND"),
    }
    Ok(vm)
}

fn register_list(vm: &mut VM) -> Result<&mut VM, Error> {
    let _ = vm.register_method(".list_init".to_string(), register_method_list_init);
    let _ = vm.register_method(".list_push".to_string(), register_method_list_push);
    let mut obj_class = Value::make_class();
    let mut super_class = Value::list();
    super_class = super_class.push(Value::from_string("Value"));
    obj_class = obj_class.set(".class_name".to_string(), Value::from_string("List"));
    obj_class = obj_class.set(".super".to_string(), super_class);
    obj_class = obj_class.set(".init".to_string(), Value::ptr(".list_init".to_string(), Vec::new()));
    obj_class = obj_class.set("push".to_string(), Value::ptr(".list_push".to_string(), Vec::new()));
    vm.register_class("List".to_string(), obj_class)
}

pub fn stdlib_object_list_value_empty(vm: &mut VM) -> Result<&mut VM, Error> {
    vm.stack.push(Value::from_list(Vec::new()));
    vm.stack.push(Value::from_string("List"));
    vm.apply(Value::call("object".to_string(), Vec::new()))
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
    match register_list(&mut bc.vm) {
        Ok(_) => {},
        Err(err) => {
            log::error!("Error registeing Bool base class: {}", err);
            bc.vm.stack.push(Value::from_int(10));
            let _ = functions::bund::bund_exit::stdlib_bund_exit_inline(&mut bc.vm);
        }
    }
    let _ = bc.vm.register_inline("List".to_string(), stdlib_object_list_value_empty);
    drop(bc);
}
