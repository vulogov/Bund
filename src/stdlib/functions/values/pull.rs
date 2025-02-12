extern crate log;
use crate::stdlib::BUND;
use rust_multistackvm::multistackvm::{VM, StackOps};
use crate::cmd;
use crate::stdlib::helpers;
use rust_dynamic::value::Value;
use easy_error::{Error, bail};

#[time_graph::instrument]
fn pull_list_base(vm: &mut VM, op: StackOps, err_prefix: String) -> Result<&mut VM, Error> {
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
    let names_value = match op {
        StackOps::FromStack => vm.stack.pull(),
        StackOps::FromWorkBench => vm.stack.pull_from_workbench(),
    };
    let names_val = match names_value {
        Some(names_val) => names_val,
        None => bail!("{} returns NO DATA #1", &err_prefix),
    };
    let names_list = match names_val.cast_list() {
        Ok(names_list) => names_list,
        Err(err) => bail!("{} casting of list returned: {}", &err_prefix, err),
    };
    let mut res = Value::dict();
    for n in names_list {
        let name = match n.cast_string() {
            Ok(name) => name,
            Err(err) => bail!("{} error casting name from string: {}", &err_prefix, err),
        };
        res = match vm.stack.pull() {
            Some(value) => res.set(name, value),
            None => bail!("{} can not pull value from stack", &err_prefix),
        };
    }
    let _ = match op {
        StackOps::FromStack => vm.stack.push(res),
        StackOps::FromWorkBench => vm.stack.push_to_workbench(res),
    };
    return Ok(vm);
}

#[time_graph::instrument]
fn pull_workbench_list_base(vm: &mut VM, op: StackOps, err_prefix: String) -> Result<&mut VM, Error> {
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
    let names_value = match op {
        StackOps::FromStack => vm.stack.pull(),
        StackOps::FromWorkBench => vm.stack.pull_from_workbench(),
    };
    let names_val = match names_value {
        Some(names_val) => names_val,
        None => bail!("{} returns NO DATA #1", &err_prefix),
    };
    let names_list = match names_val.cast_list() {
        Ok(names_list) => names_list,
        Err(err) => bail!("{} casting of list returned: {}", &err_prefix, err),
    };
    let mut res = Value::dict();
    for n in names_list {
        let name = match n.cast_string() {
            Ok(name) => name,
            Err(err) => bail!("{} error casting name from string: {}", &err_prefix, err),
        };
        res = match vm.stack.pull_from_workbench() {
            Some(value) => res.set(name, value),
            None => bail!("{} can not pull value from stack", &err_prefix),
        };
    }
    let _ = match op {
        StackOps::FromStack => vm.stack.push(res),
        StackOps::FromWorkBench => vm.stack.push_to_workbench(res),
    };
    return Ok(vm);
}

pub fn stdlib_pull_list_stack(vm: &mut VM) -> Result<&mut VM, Error> {
    pull_list_base(vm, StackOps::FromStack, "PULL".to_string())
}
pub fn stdlib_pull_list_workbench(vm: &mut VM) -> Result<&mut VM, Error> {
    pull_list_base(vm, StackOps::FromWorkBench, "PULL.".to_string())
}

pub fn stdlib_pull_workbench_list_stack(vm: &mut VM) -> Result<&mut VM, Error> {
    pull_workbench_list_base(vm, StackOps::FromStack, "PULL.WORKBENCH".to_string())
}
pub fn stdlib_pull_workbench_list_workbench(vm: &mut VM) -> Result<&mut VM, Error> {
    pull_workbench_list_base(vm, StackOps::FromWorkBench, "PULL.WORKBENCH.".to_string())
}


pub fn init_stdlib(cli: &cmd::Cli) {
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };

    let _ = bc.vm.register_inline("pull".to_string(), stdlib_pull_list_stack);
    let _ = bc.vm.register_inline("pull.".to_string(), stdlib_pull_list_workbench);
    let _ = bc.vm.register_inline("pull.workbench".to_string(), stdlib_pull_workbench_list_stack);
    let _ = bc.vm.register_inline("pull.workbench.".to_string(), stdlib_pull_workbench_list_workbench);

    drop(bc);
}
