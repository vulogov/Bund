extern crate log;
use crate::cmd;
use crate::stdlib::BUND;
use rust_dynamic::types::*;
use rust_dynamic::value::Value;
use rust_multistackvm::multistackvm::VM;
use crate::stdlib::{helpers, functions};
use crate::stdlib::functions::oop;
use easy_error::{Error, bail};

fn register_method_float_init(vm: &mut VM) -> Result<&mut VM, Error> {
    let value_object = match vm.stack.pull() {
        Some(value_object) => value_object,
        None => bail!("Float: stack is empty"),
    };
    match oop::value_class::locate_value_in_object(".data".to_string(), value_object.clone()) {
        Some(data_object) => {
            match data_object.conv(FLOAT) {
                Ok(new_data_object) => {
                    let new_value_object = oop::value_class::set_value_in_object(".data".to_string(), value_object, new_data_object);
                    vm.stack.push(new_value_object);
                }
                Err(err) => bail!("Float: error converting to FLOAT: {}", err),
            }
        }
        None => bail!("Float: NO WRAPPED DATA WAS FOUND"),
    }
    Ok(vm)
}

fn register_float(vm: &mut VM) -> Result<&mut VM, Error> {
    let _ = vm.register_method(".float_init".to_string(), register_method_float_init);
    let mut obj_class = Value::make_class();
    let mut super_class = Value::list();
    super_class = super_class.push(Value::from_string("Value"));
    obj_class = obj_class.set(".class_name".to_string(), Value::from_string("Float"));
    obj_class = obj_class.set(".super".to_string(), super_class);
    obj_class = obj_class.set(".init".to_string(), Value::ptr(".float_init".to_string(), Vec::new()));
    vm.register_class("Float".to_string(), obj_class)
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
    match register_float(&mut bc.vm) {
        Ok(_) => {},
        Err(err) => {
            log::error!("Error registeing Float base class: {}", err);
            bc.vm.stack.push(Value::from_int(10));
            let _ = functions::bund::bund_exit::stdlib_bund_exit_inline(&mut bc.vm);
        }
    }
    drop(bc);
}
