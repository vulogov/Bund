extern crate log;
use crate::stdlib::BUND;
use rust_multistackvm::multistackvm::{VM, StackOps};
use rust_dynamic::value::Value;
use crate::cmd;
use crate::stdlib::helpers;
use easy_error::{Error, bail};
use fancy_regex::{Regex};

#[time_graph::instrument]
pub fn string_regex_base(vm: &mut VM, op: StackOps, err_prefix: String) -> Result<&mut VM, Error> {
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
            if vm.stack.current_stack_len() < 1 {
                bail!("Stack is too shallow for inline {}", &err_prefix);
            }
        }
    }
    let string_val = match op {
        StackOps::FromStack => vm.stack.pull(),
        StackOps::FromWorkBench => vm.stack.pull_from_workbench(),
    };
    let pattern_val = match op {
        StackOps::FromStack => vm.stack.pull(),
        StackOps::FromWorkBench => vm.stack.pull(),
    };
    match string_val {
        Some(string_val) => {
            match string_val.cast_string() {
                Ok(string_data) => {
                    match pattern_val {
                        Some(pattern_val) => {
                            match pattern_val.cast_string() {
                                Ok(pattern) => {
                                    match Regex::new(&pattern) {
                                        Ok(regx) => {
                                            match regx.is_match(&string_data) {
                                                Ok(res) => {
                                                    vm.stack.push(Value::from_bool(res));
                                                }
                                                Err(err) => {
                                                    bail!("{} matching returns: {}", &err_prefix, err);
                                                }
                                            }
                                        }
                                        Err(err) => {
                                            bail!("{} compile returns: {}", &err_prefix, err);
                                        }
                                    }
                                }
                                Err(err) => {
                                    bail!("{} returns: {}", &err_prefix, err);
                                }
                            }
                        }
                        None => {
                            bail!("{} returns: NO DATA #2", &err_prefix);
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


pub fn stdlib_string_stack_regex_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    string_regex_base(vm, StackOps::FromStack, "STRING.REGEX".to_string())
}

pub fn stdlib_string_workbench_regex_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    string_regex_base(vm, StackOps::FromWorkBench, "STRING.REGEX.".to_string())
}


pub fn init_stdlib(cli: &cmd::Cli) {
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };
    let _ = bc.vm.register_inline("string.regex".to_string(), stdlib_string_stack_regex_inline);
    let _ = bc.vm.register_inline("string.regex.".to_string(), stdlib_string_workbench_regex_inline);

    drop(bc);
}
