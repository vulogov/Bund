extern crate log;
use crate::cmd;
use crate::stdlib::BUND;
use rust_dynamic::types::*;
use rust_dynamic::value::Value;
use rust_multistackvm::multistackvm::VM;
use crate::stdlib::{helpers, functions};
use std::collections::HashMap;
use leon::{Template};
use termimad::{term_text};
use easy_error::{Error, bail};


fn register_method_format(vm: &mut VM) -> Result<&mut VM, Error> {
    log::debug!("'.format' is called");
    let value = match vm.stack.pull() {
        Some(value) => value,
        None => bail!("Stack is empty for method '.format'"),
    };
    match value.type_of() {
        OBJECT => {
            let template_str = match value.get(".template") {
                Ok(template_value) => match template_value.cast_string() {
                    Ok(template_str) => template_str,
                    Err(err) => bail!("'.format' error casting template: {}", err),
                },
                Err(err) => bail!("'.format' NO TEMPLATE: {}", err),
            };
            let template = match Template::parse(template_str.as_str()) {
                Ok(template) => template,
                Err(err) => {
                    bail!("FMT.STR error parsing template: {}", err);
                }
            };
            let mut values: HashMap<String, String> = HashMap::new();
            for name in template.keys() {
                if values.contains_key(&name.to_string().clone()) {
                    continue;
                }
                let attr_value = match value.get(name) {
                    Ok(attr_value) => attr_value,
                    Err(_) => match vm.stack.pull() {
                        Some(attr_value) => attr_value,
                        None => bail!("'.format' can not resolve {}", &name),
                    },
                };
                match attr_value.conv(STRING) {
                    Ok(str_value) => {
                        match str_value.cast_string() {
                            Ok(attr_str) => {
                                values.insert(name.to_string(), attr_str);
                            }
                            Err(err) => bail!("'.format' error casting value to STRING: {}", err),
                        }
                    }
                    Err(err) => bail!("'.format' error converting value to STRING: {}", err),
                }
            }
            let res = match template.render(&values) {
                Ok(res) => res.to_string(),
                Err(err) => {
                    bail!("FMT.STR error rendering: {}", err);
                }
            };
            let out = Value::from_string(format!("{}", term_text(&res)));
            //
            // And pushing ready output
            //
            vm.stack.push(value);
            vm.stack.push(out);
        }
        _ => {
            match value.conv(STRING) {
                Ok(str_value) => {
                    vm.stack.push(value);
                    vm.stack.push(str_value);
                }
                Err(err) => {
                    bail!("'.format' returns: {}", err);
                }
            }
        }
    }
    Ok(vm)
}

fn register_method_display(vm: &mut VM) -> Result<&mut VM, Error> {
    log::debug!("'.display' is called");
    match register_method_format(vm) {
        Ok(_) => {},
        Err(err) => bail!("{}", err),
    };
    rust_multistackvm::stdlib::print::stdlib_print_inline(vm)
}

fn register_display(vm: &mut VM) -> Result<&mut VM, Error> {
    let _ = vm.register_method(".format".to_string(), register_method_format);
    let _ = vm.register_method(".display".to_string(), register_method_display);
    let mut obj_class = Value::make_class();
    let super_class = Value::list();
    obj_class = obj_class.set(".class_name".to_string(), Value::from_string("Display"));
    obj_class = obj_class.set(".super".to_string(), super_class);
    obj_class = obj_class.set("format".to_string(), Value::ptr(".format".to_string(), Vec::new()));
    obj_class = obj_class.set("display".to_string(), Value::ptr(".display".to_string(), Vec::new()));
    vm.register_class("Display".to_string(), obj_class)
}

pub fn init_stdlib(cli: &cmd::Cli) {
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };

    match register_display(&mut bc.vm) {
        Ok(_) => {},
        Err(err) => {
            log::error!("Error registeing Display base class: {}", err);
            bc.vm.stack.push(Value::from_int(10));
            let _ = functions::bund::bund_exit::stdlib_bund_exit_inline(&mut bc.vm);
        }
    }

    drop(bc);
}
