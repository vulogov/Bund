extern crate log;
use crate::cmd;
use crate::stdlib::BUND;
use rust_multistackvm::multistackvm::{VM, StackOps};
use crate::stdlib::helpers;
use rust_dynamic::types::*;
use rust_dynamic::value::Value;
use easy_error::{Error, bail};

#[time_graph::instrument]
fn stdlib_object_execute_base(vm: &mut VM, depth: usize, op: StackOps, err_prefix: String) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < depth {
        bail!("Stack is too shallow for inline {}", &err_prefix);
    }
    let lambda_val = match vm.stack.pull() {
        Some(lambda_val) => match lambda_val.type_of() {
            LAMBDA => lambda_val,
            PTR => lambda_val,
            _ => bail!("{} NO LAMBDA or PTR IN #1", &err_prefix),
        },
        None => bail!("{} NO DATA IN #1", &err_prefix),
    };
    let object_val = match op {
        StackOps::FromStack => vm.stack.pull(),
        StackOps::FromWorkBench => vm.stack.pull_from_workbench(),
    };
    let obj_val = match object_val {
        Some(obj_val) => if obj_val.type_of() == OBJECT {
            obj_val
        } else {
            bail!("{} NO OBJECT IN #2", &err_prefix);
        },
        None => bail!("{} NO DATA IN #2", &err_prefix),
    };
    vm.stack.push(obj_val.clone());
    let _ = vm.apply(Value::call("unwrap".to_string(), Vec::new()));
    vm.stack.push(lambda_val.clone());
    let _ = vm.apply(Value::call("!".to_string(), Vec::new()));
    Ok(vm)
}

pub fn stdlib_object_execute(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_object_execute_base(vm, 2, StackOps::FromStack, "#".to_string())
}

pub fn stdlib_object_execute_in_workbench(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_object_execute_base(vm, 1, StackOps::FromWorkBench, "#.".to_string())
}


pub fn init_stdlib(cli: &cmd::Cli) {
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };
    let _ = bc.vm.register_inline("#".to_string(), stdlib_object_execute);
    let _ = bc.vm.register_inline("#.".to_string(), stdlib_object_execute_in_workbench);
    drop(bc);
}
