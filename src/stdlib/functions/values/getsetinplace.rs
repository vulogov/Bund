extern crate log;
use crate::stdlib::BUND;
use rust_multistackvm::multistackvm::{VM, StackOps};
use crate::cmd;
use crate::stdlib::helpers;
use rust_dynamic::types::*;
use easy_error::{Error, bail};

#[derive(Debug, Clone)]
pub enum DictOp {
    GetInplace,
    SetInplace,
}

#[time_graph::instrument]
fn stdlib_inplacegetset_base(vm: &mut VM, op: StackOps, dop: DictOp, err_prefix: String) -> Result<&mut VM, Error> {
    match op {
        StackOps::FromStack => {
            match dop {
                DictOp::GetInplace => if vm.stack.current_stack_len() < 2 {
                    bail!("Stack is too shallow for inline {}", &err_prefix);
                },
                DictOp::SetInplace => if vm.stack.current_stack_len() < 3 {
                    bail!("Stack is too shallow for inline {}", &err_prefix);
                },
            }
        }
        StackOps::FromWorkBench => {
            if vm.stack.workbench.len() < 1 {
                bail!("Workbench is too shallow for inline {}", &err_prefix);
            }
            match dop {
                DictOp::GetInplace => if vm.stack.current_stack_len() < 1 {
                    bail!("Stack is too shallow for inline {}", &err_prefix);
                },
                DictOp::SetInplace => if vm.stack.current_stack_len() < 2 {
                    bail!("Stack is too shallow for inline {}", &err_prefix);
                },
            }
        }
    }
    let dict_value = match op {
        StackOps::FromStack => vm.stack.pull(),
        StackOps::FromWorkBench => vm.stack.pull_from_workbench(),
    };
    let mut dict_val = match dict_value {
        Some(dict_val) => dict_val,
        None => bail!("{} returns NO DATA #1", &err_prefix),
    };
    match dict_val.dt {
        MAP | INFO | CONFIG | ASSOCIATION | CURRY | MESSAGE | CONDITIONAL | OBJECT  => {
            match dop {
                DictOp::GetInplace => {
                    let key_val = match vm.stack.pull() {
                        Some(key_val) => key_val,
                        None => bail!("{} returns NO DATA #2", &err_prefix),
                    };
                    let key = match key_val.cast_string() {
                        Ok(key) => key,
                        Err(err) => bail!("{} error in GET: {}", &err_prefix, err),
                    };
                    let val = match dict_val.get(&key) {
                        Ok(val) => val,
                        Err(err) => bail!("{}: Key not found: {} due to: {}", &err_prefix, &key, err),
                    };
                    let _ = match op {
                        StackOps::FromStack => vm.stack.push(dict_val),
                        StackOps::FromWorkBench => vm.stack.push_to_workbench(dict_val),
                    };
                    vm.stack.push(val);
                }
                DictOp::SetInplace => {
                    let obj_val = match vm.stack.pull() {
                        Some(obj_val) => obj_val,
                        None => bail!("{} returns NO DATA #1", &err_prefix),
                    };
                    let key_val = match vm.stack.pull() {
                        Some(key_val) => key_val,
                        None => bail!("{} returns NO DATA #2", &err_prefix),
                    };
                    let key = match key_val.cast_string() {
                        Ok(key) => key,
                        Err(err) => bail!("{} error in GET: {}", &err_prefix, err),
                    };
                    dict_val = dict_val.set(&key, obj_val);
                    let _ = match op {
                        StackOps::FromStack => vm.stack.push(dict_val),
                        StackOps::FromWorkBench => vm.stack.push_to_workbench(dict_val),
                    };
                }
            }
        }
        _ => {
            bail!("{}: unsupported operation for a DICT-based type", &err_prefix);
        }
    }
    return Ok(vm);
}

pub fn stdlib_get_inline_stack(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_inplacegetset_base(vm, StackOps::FromStack, DictOp::GetInplace, "GET,".to_string())
}

pub fn stdlib_get_inline_workbench(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_inplacegetset_base(vm, StackOps::FromWorkBench, DictOp::GetInplace, "GET.,".to_string())
}

pub fn stdlib_set_inline_stack(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_inplacegetset_base(vm, StackOps::FromStack, DictOp::SetInplace, "SET,".to_string())
}

pub fn stdlib_set_inline_workbench(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_inplacegetset_base(vm, StackOps::FromWorkBench, DictOp::SetInplace, "SET.,".to_string())
}

pub fn init_stdlib(cli: &cmd::Cli) {
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };

    let _ = bc.vm.register_inline("get,".to_string(), stdlib_get_inline_stack);
    let _ = bc.vm.register_inline("get.,".to_string(), stdlib_get_inline_workbench);
    let _ = bc.vm.register_inline("set,".to_string(), stdlib_set_inline_stack);
    let _ = bc.vm.register_inline("set.,".to_string(), stdlib_set_inline_workbench);

    drop(bc);
}
