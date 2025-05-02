extern crate log;
use crate::stdlib::BUND;
use rust_multistackvm::multistackvm::{VM};
use crate::cmd;
use rust_dynamic::types::*;
use crate::stdlib::{helpers, functions};
use crate::stdlib::functions::oop;
use rust_dynamic::value::Value;
use easy_error::{Error, bail};


fn register_method_ollama_init(vm: &mut VM) -> Result<&mut VM, Error> {
    let mut value_object = match vm.stack.pull() {
        Some(value_object) => value_object,
        None => bail!("OLLAMA: stack is empty"),
    };
    match oop::value_class::locate_value_in_object(".data".to_string(), value_object.clone()) {
        Some(data_object) => {
            match data_object.type_of() {
                MAP => {
                    let addr = match data_object.get("address") {
                        Ok(addr) => match addr.cast_string() {
                            Ok(addr) => addr,
                            Err(err) => bail!("OLLAMA: error casting address: {}", err),
                        },
                        Err(_) => "127.0.0.1".to_string(),
                    };
                    value_object = oop::value_class::set_value_in_object(".data".to_string(), value_object, Value::from_string(&addr));
                    vm.stack.push(value_object);
                }
                STRING => {
                    let data = data_object.cast_string().unwrap();
                    let new_value_object = oop::value_class::set_value_in_object(".data".to_string(), value_object, Value::from_string(&data));
                    vm.stack.push(new_value_object);
                }
                _ => bail!("OLLAMA: WRAPPED TYPE IS NOT SUPPORTED: {}", &data_object.type_name()),
            }
        }
        None => bail!("OLLAMA: NO WRAPPED DATA WAS FOUND FOR INIT"),
    }
    Ok(vm)
}

fn register_ollama(vm: &mut VM) -> Result<&mut VM, Error> {
    let _ = vm.register_method(".ollama_init".to_string(), register_method_ollama_init);
    let mut obj_class = Value::make_class();
    let mut super_class = Value::list();
    super_class = super_class.push(Value::from_string("Value"));
    super_class = super_class.push(Value::from_string("Display"));
    obj_class = obj_class.set(".class_name".to_string(), Value::from_string("OLLAMA"));
    obj_class = obj_class.set(".super".to_string(), super_class);
    obj_class = obj_class.set(".init".to_string(), Value::ptr(".ollama_init".to_string(), Vec::new()));
    vm.register_class("OLLAMA".to_string(), obj_class)
}

pub fn init_stdlib(cli: &cmd::Cli) {
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };

    match register_ollama(&mut bc.vm) {
        Ok(_) => {},
        Err(err) => {
            log::error!("Error registeing OLLAMA base class: {}", err);
            bc.vm.stack.push(Value::from_int(10));
            let _ = functions::bund::bund_exit::stdlib_bund_exit_inline(&mut bc.vm);
        }
    }

    drop(bc);
}
