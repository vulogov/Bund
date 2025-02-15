extern crate log;
use crate::stdlib::BUND;
use rust_multistackvm::multistackvm::{VM, StackOps};
use crate::cmd;
use crate::stdlib::helpers;
use rust_dynamic::value::Value;
use algos::cs::search::fibonacci;
use easy_error::{Error, bail};

#[time_graph::instrument]
fn unique_list_base(vm: &mut VM, op: StackOps, err_prefix: String) -> Result<&mut VM, Error> {
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
    let list_value = match op {
        StackOps::FromStack => vm.stack.pull(),
        StackOps::FromWorkBench => vm.stack.pull_from_workbench(),
    };
    let list_val = match list_value {
        Some(list_val) => list_val,
        None => bail!("{} returns NO DATA #1", &err_prefix),
    };
    let raw_list = match list_val.cast_list() {
        Ok(raw_list) => raw_list,
        Err(err) => bail!("{} casting of list returned: {}", &err_prefix, err),
    };
    let mut res: Vec<Value> = Vec::new();
    for i in raw_list {
        match fibonacci::search(&res, &i) {
            Ok(Some(_)) => continue,
            Ok(None) => {
                res.push(i);
            }
            Err(err) => bail!("{} returns error during the scan: {}", &err_prefix, err),
        };
    }
    let _ = match op {
        StackOps::FromStack => vm.stack.push(Value::from_list(res)),
        StackOps::FromWorkBench => vm.stack.push_to_workbench(Value::from_list(res)),
    };
    return Ok(vm);
}

pub fn stdlib_unique_list_stack(vm: &mut VM) -> Result<&mut VM, Error> {
    unique_list_base(vm, StackOps::FromStack, "UNIQUE".to_string())
}

pub fn stdlib_unique_list_workbench(vm: &mut VM) -> Result<&mut VM, Error> {
    unique_list_base(vm, StackOps::FromWorkBench, "UNIQUE.".to_string())
}

pub fn init_stdlib(cli: &cmd::Cli) {
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };

    let _ = bc.vm.register_inline("unique".to_string(), stdlib_unique_list_stack);
    let _ = bc.vm.register_inline("unique.".to_string(), stdlib_unique_list_workbench);

    drop(bc);
}
