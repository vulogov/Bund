extern crate log;
use crate::cmd;
use crate::stdlib::BUND;
use rust_dynamic::types::*;
use rust_dynamic::value::Value;
use rust_multistackvm::multistackvm::VM;
use crate::stdlib::{helpers, functions};
use crate::stdlib::functions::oop;
use easy_error::{Error, bail};

fn register_method_float_list_init(vm: &mut VM) -> Result<&mut VM, Error> {
    let value_object = match vm.stack.pull() {
        Some(value_object) => value_object,
        None => bail!("Floats: stack is empty"),
    };
    match oop::value_class::locate_value_in_object(".data".to_string(), value_object.clone()) {
        Some(data_object) => {
            let mut new_data = Value::list();
            for v in data_object {
                match v.conv(FLOAT) {
                    Ok(new_v) => new_data = new_data.push(new_v),
                    Err(err) => bail!("Floats: error casting floating data: {}", err),
                }
            }
            let new_value_object = oop::value_class::set_value_in_object(".data".to_string(), value_object, new_data);
            vm.stack.push(new_value_object);
        }
        None => bail!("List: NO WRAPPED DATA WAS FOUND"),
    }
    Ok(vm)
}

fn register_method_floats_push(vm: &mut VM) -> Result<&mut VM, Error> {
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
                let new_float_object = match new_object.conv(FLOAT) {
                    Ok(new_float_object) => new_float_object,
                    Err(err) => bail!("Flaots::push data object is not FLOAT: {}", err),
                };
                data_object = data_object.push(new_float_object);
            } else {
                bail!("Floats::push data object is not LIST");
            }
            let new_value_object = oop::value_class::set_value_in_object(".data".to_string(), value_object, data_object);
            vm.stack.push(new_value_object);
        }
        None => bail!("Floats: NO WRAPPED DATA WAS FOUND"),
    }
    Ok(vm)
}

fn register_floats(vm: &mut VM) -> Result<&mut VM, Error> {
    let _ = vm.register_method(".floats_init".to_string(), register_method_float_list_init);
    let _ = vm.register_method(".floats_push".to_string(), register_method_floats_push);
    let mut obj_class = Value::make_class();
    let mut super_class = Value::list();
    super_class = super_class.push(Value::from_string("List"));
    obj_class = obj_class.set(".class_name".to_string(), Value::from_string("Floats"));
    obj_class = obj_class.set(".super".to_string(), super_class);
    obj_class = obj_class.set(".init".to_string(), Value::ptr(".floats_init".to_string(), Vec::new()));
    obj_class = obj_class.set("push".to_string(), Value::ptr(".floats_push".to_string(), Vec::new()));
    vm.register_class("Floats".to_string(), obj_class)
}

pub fn stdlib_object_floats_value_empty(vm: &mut VM) -> Result<&mut VM, Error> {
    vm.stack.push(Value::from_list(Vec::new()));
    vm.stack.push(Value::from_string("Floats"));
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
    match register_floats(&mut bc.vm) {
        Ok(_) => {},
        Err(err) => {
            log::error!("Error registeing Floats base class: {}", err);
            bc.vm.stack.push(Value::from_int(10));
            let _ = functions::bund::bund_exit::stdlib_bund_exit_inline(&mut bc.vm);
        }
    }
    let _ = bc.vm.register_inline("Floats".to_string(), stdlib_object_floats_value_empty);
    drop(bc);
}
