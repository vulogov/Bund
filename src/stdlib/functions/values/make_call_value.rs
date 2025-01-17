extern crate log;
use crate::stdlib::BUND;
use rust_multistackvm::multistackvm::{VM, StackOps};
use crate::cmd;
use crate::stdlib::helpers;
use rust_dynamic::value::Value;
use easy_error::{Error, bail};

#[time_graph::instrument]
fn make_call_value_base(vm: &mut VM, op: StackOps, err_prefix: String) -> Result<&mut VM, Error> {
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
    let name_value = match op {
        StackOps::FromStack => vm.stack.pull(),
        StackOps::FromWorkBench => vm.stack.pull_from_workbench(),
    };
    let name_val = match name_value {
        Some(name_val) => name_val,
        None => bail!("{} returns NO DATA #1", &err_prefix),
    };
    let name = match name_val.cast_string() {
        Ok(name) => name,
        Err(err) => bail!("{} casting of string returned: {}", &err_prefix, err),
    };

    let _ = match op {
        StackOps::FromStack => vm.stack.push(Value::call(name, Vec::new())),
        StackOps::FromWorkBench => vm.stack.push_to_workbench(Value::call(name, Vec::new())),
    };
    return Ok(vm);
}

pub fn stdlib_make_call_stack(vm: &mut VM) -> Result<&mut VM, Error> {
    make_call_value_base(vm, StackOps::FromStack, "MAKE.CALL".to_string())
}

pub fn stdlib_make_call_workbench(vm: &mut VM) -> Result<&mut VM, Error> {
    make_call_value_base(vm, StackOps::FromWorkBench, "MAKE.CALL.".to_string())
}


pub fn init_stdlib(cli: &cmd::Cli) {
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };

    let _ = bc.vm.register_inline("make.call".to_string(), stdlib_make_call_stack);
    let _ = bc.vm.register_inline("make.call.".to_string(), stdlib_make_call_workbench);

    drop(bc);
}
