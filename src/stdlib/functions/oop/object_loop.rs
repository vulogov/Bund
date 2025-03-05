extern crate log;
use crate::cmd;
use crate::stdlib::BUND;
use rust_multistackvm::multistackvm::{VM, StackOps};
use crate::stdlib::helpers;
use rust_dynamic::types::*;
use easy_error::{Error, bail};

#[time_graph::instrument]
fn stdlib_object_loop_base(vm: &mut VM, depth: usize, op: StackOps, err_prefix: String) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < depth {
        bail!("Stack is too shallow for inline {}", &err_prefix);
    }
    let _lambda_val = match vm.stack.pull() {
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
    let _obj_val = match object_val {
        Some(obj_val) => if obj_val.type_of() == OBJECT {
            obj_val
        } else {
            bail!("{} NO OBJECT IN #2", &err_prefix);
        },
        None => bail!("{} NO DATA IN #2", &err_prefix),
    };
    Ok(vm)
}

pub fn stdlib_object_loop(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_object_loop_base(vm, 2, StackOps::FromStack, "!LOOP".to_string())
}

pub fn stdlib_object_loop_in_workbench(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_object_loop_base(vm, 1, StackOps::FromWorkBench, "!LOOP.".to_string())
}


pub fn init_stdlib(cli: &cmd::Cli) {
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };
    let _ = bc.vm.register_inline("!loop".to_string(), stdlib_object_loop);
    let _ = bc.vm.register_inline("!loop.".to_string(), stdlib_object_loop_in_workbench);
    drop(bc);
}
