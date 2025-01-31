extern crate log;
use crate::stdlib::BUND;
use rust_multistackvm::multistackvm::{VM, StackOps};
use rust_dynamic::value::Value;
use crate::cmd;
use crate::stdlib::helpers;
use easy_error::{bail,Error};
use deunicode;

#[derive(Debug, Clone)]
pub enum UnicodeAlgorithm {
    Deunicode,
}

#[time_graph::instrument]
fn string_unicode_base(vm: &mut VM, op: StackOps, ta:UnicodeAlgorithm, err_prefix: String) -> Result<&mut VM, Error> {
    match op {
        StackOps::FromStack => {
            if vm.stack.current_stack_len() < 1 {
                bail!("Stack is too shallow for inline {}", &err_prefix);
            }
        }
        StackOps::FromWorkBench => {
            if vm.stack.workbench.len() < 1 {
                bail!("Workbench is too shallow for inline {}", &err_prefix);
            }
        }
    }
    let string_val = match op {
        StackOps::FromStack => vm.stack.pull(),
        StackOps::FromWorkBench => vm.stack.pull_from_workbench(),
    };
    match string_val {
        Some(string_val) => {
            match string_val.cast_string() {
                Ok(string_data) => {
                    match ta {
                        UnicodeAlgorithm::Deunicode => {
                            match op {
                                StackOps::FromStack => vm.stack.push(Value::from_string(deunicode::deunicode(&string_data))),
                                StackOps::FromWorkBench => vm.stack.push_to_workbench(Value::from_string(deunicode::deunicode(&string_data))),
                            };
                        }
                    }
                }
                Err(err) => {
                    bail!("{} returns: {}", &err_prefix, err);
                }
            }
        }
        None => {
            bail!("{} returns: NO DATA #1", &err_prefix);
        }
    }
    Ok(vm)
}

pub fn stdlib_unicode_de_wb_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    string_unicode_base(vm, StackOps::FromWorkBench, UnicodeAlgorithm::Deunicode, "STRING.DEUNICODE.".to_string() )
}
pub fn stdlib_unicode_de_stack_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    string_unicode_base(vm, StackOps::FromStack, UnicodeAlgorithm::Deunicode, "STRING.DEUNICODE".to_string() )
}


pub fn init_stdlib(cli: &cmd::Cli) {
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };
    let _ = bc.vm.register_inline("string.deunicode".to_string(), stdlib_unicode_de_stack_inline);
    let _ = bc.vm.register_inline("string.deunicode.".to_string(), stdlib_unicode_de_wb_inline);

    drop(bc);
}
