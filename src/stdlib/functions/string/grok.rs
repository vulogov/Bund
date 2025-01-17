extern crate log;
use crate::stdlib::BUND;
use rust_multistackvm::multistackvm::{VM, StackOps};
use rust_dynamic::value::Value;
use crate::cmd;
use crate::stdlib::helpers;
use easy_error::{Error, bail};
use grok::Grok;

#[time_graph::instrument]
pub fn string_grok_base(vm: &mut VM, op: StackOps, err_prefix: String) -> Result<&mut VM, Error> {
    match op {
        StackOps::FromStack => {
            if vm.stack.current_stack_len() < 2 {
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
                                    let mut res = Value::dict();
                                    let mut grok = Grok::with_default_patterns();
                                    match grok.compile(&pattern, false) {
                                        Ok(patt) => {
                                            match patt.match_against(&string_data) {
                                                Some(m) => {
                                                    for (k,v) in &m {
                                                        if ! &v.is_empty() {
                                                            res = res.set(&k, Value::from_string(&v));
                                                        }
                                                    }
                                                }
                                                None => {
                                                    log::debug!("Pattern {} doesnt matching anything", &pattern);
                                                }
                                            }
                                        }
                                        Err(err) => {
                                            bail!("{} error while compiling the pattern: {}", &err_prefix, err);
                                        }
                                    }
                                    vm.stack.push(res);
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


pub fn stdlib_string_stack_grok_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    string_grok_base(vm, StackOps::FromStack, "STRING.GROK".to_string())
}

pub fn stdlib_string_workbench_grok_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    string_grok_base(vm, StackOps::FromWorkBench, "STRING.GROK.".to_string())
}


pub fn init_stdlib(cli: &cmd::Cli) {
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };
    let _ = bc.vm.register_inline("string.grok".to_string(), stdlib_string_stack_grok_inline);
    let _ = bc.vm.register_inline("string.grok.".to_string(), stdlib_string_workbench_grok_inline);

    drop(bc);
}
