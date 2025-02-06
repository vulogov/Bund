extern crate log;
use crate::stdlib::BUND;
use rust_multistackvm::multistackvm::{VM, StackOps};
use crate::cmd;
use crate::stdlib::helpers;
use rust_dynamic::value::Value;
use rust_dynamic::types::*;
use rust_dynamic::math;
use easy_error::{Error, bail};

fn default_math_op(vm: &mut VM, op: StackOps, x: Value, y: Value, err_prefix: String) -> Result<&mut VM, Error> {
    let res = match Value::numeric_op(math::Ops::Add, x, y) {
        Ok(res) => res,
        Err(err) => bail!("{} returns error for default operation: {}", &err_prefix, err),
    };
    let _ = match op {
        StackOps::FromStack => vm.stack.push(res),
        StackOps::FromWorkBench => vm.stack.push_to_workbench(res),
    };
    Ok(vm)
}

#[time_graph::instrument]
fn merge_base(vm: &mut VM, op: StackOps, err_prefix: String) -> Result<&mut VM, Error> {
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
                bail!("Workbench is too shallow for inline {}", &err_prefix);
            }
        }
    }
    let op1_value = match op {
        StackOps::FromStack => vm.stack.pull(),
        StackOps::FromWorkBench => vm.stack.pull_from_workbench(),
    };
    let mut op_val1 = match op1_value {
        Some(list_val) => list_val,
        None => bail!("{} returns NO DATA #1", &err_prefix),
    };
    let op2_value = vm.stack.pull();
    let op_val2 = match op2_value {
        Some(list_val) => list_val,
        None => bail!("{} returns NO DATA #2", &err_prefix),
    };
    log::debug!("{}: merging {} <- {}", &err_prefix, op_val1.type_name(), op_val2.type_name());
    match op_val1.dt {
        MAP | CONDITIONAL => {
            match op_val2.dt {
                LIST => {
                    let raw_list = match op_val2.cast_list() {
                        Ok(raw_list) => raw_list,
                        Err(err) => bail!("{} returns error during LIST casting: {}", err_prefix, err),
                    };
                    let mut c: usize = 0;
                    for v in raw_list {
                        op_val1 = op_val1.set(format!("{}", c), v.clone());
                        c += 1;
                    }
                    let _ = match op {
                        StackOps::FromStack => vm.stack.push(op_val1),
                        StackOps::FromWorkBench => vm.stack.push_to_workbench(op_val1),
                    };
                    return Ok(vm);
                }
                MAP | CONDITIONAL => {
                    let o_type = op_val1.dt;
                    let raw_map = match op_val2.cast_dict() {
                        Ok(raw_map) => raw_map,
                        Err(err) => bail!("{} returns error during DICT casting: {}", err_prefix, err),
                    };
                    for (k,v) in raw_map {
                        op_val1 = op_val1.set(k, v.clone());

                    }
                    op_val1.dt = o_type;
                    let _ = match op {
                        StackOps::FromStack => vm.stack.push(op_val1),
                        StackOps::FromWorkBench => vm.stack.push_to_workbench(op_val1),
                    };
                    return Ok(vm);
                }
                _ => {
                    return default_math_op(vm, op, op_val1, op_val2, err_prefix);
                }
            }
        }
        _ => {
            match op_val2.dt {
                _ => {
                    return default_math_op(vm, op, op_val1, op_val2, err_prefix);
                }
            }
        }
    }
}

pub fn stdlib_merge_stack(vm: &mut VM) -> Result<&mut VM, Error> {
    merge_base(vm, StackOps::FromStack, "MERGE".to_string())
}

pub fn stdlib_merge_workbench(vm: &mut VM) -> Result<&mut VM, Error> {
    merge_base(vm, StackOps::FromWorkBench, "MERGE.".to_string())
}


pub fn init_stdlib(cli: &cmd::Cli) {
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };

    let _ = bc.vm.register_inline("merge".to_string(), stdlib_merge_stack);
    let _ = bc.vm.register_inline("merge.".to_string(), stdlib_merge_workbench);

    drop(bc);
}
