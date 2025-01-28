extern crate log;
use crate::stdlib::BUND;
use rust_multistackvm::multistackvm::{VM, StackOps};
use crate::cmd;
use crate::stdlib::helpers;
use rust_dynamic::types::*;
use easy_error::{Error, bail};

#[time_graph::instrument]
fn push_list_base(vm: &mut VM, op: StackOps, err_prefix: String) -> Result<&mut VM, Error> {
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
    let list_val1 = match op1_value {
        Some(list_val) => list_val,
        None => bail!("{} returns NO DATA #1", &err_prefix),
    };
    let mut raw_list1 = match list_val1.conv(LIST) {
        Ok(raw_list) => raw_list,
        Err(err) => bail!("{} casting of list returned: {}", &err_prefix, err),
    };
    let op2_value = vm.stack.pull();
    let list_val2 = match op2_value {
        Some(list_val) => list_val,
        None => bail!("{} returns NO DATA #2", &err_prefix),
    };
    let raw_list2 = match list_val2.conv(LIST) {
        Ok(raw_list) => raw_list,
        Err(err) => bail!("{} casting of list returned: {}", &err_prefix, err),
    };

    raw_list1 = raw_list1.push(raw_list2.clone());
    let _ = match op {
        StackOps::FromStack => vm.stack.push(raw_list1),
        StackOps::FromWorkBench => vm.stack.push_to_workbench(raw_list1),
    };
    return Ok(vm);
}

pub fn stdlib_push_list_stack(vm: &mut VM) -> Result<&mut VM, Error> {
    push_list_base(vm, StackOps::FromStack, "PUSH".to_string())
}

pub fn stdlib_push_list_workbench(vm: &mut VM) -> Result<&mut VM, Error> {
    push_list_base(vm, StackOps::FromWorkBench, "PUSH.".to_string())
}


pub fn init_stdlib(cli: &cmd::Cli) {
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };

    let _ = bc.vm.register_inline("push".to_string(), stdlib_push_list_stack);
    let _ = bc.vm.register_inline("push.".to_string(), stdlib_push_list_workbench);

    drop(bc);
}
