extern crate log;
use crate::stdlib::BUND;
use rust_multistackvm::multistackvm::{VM, StackOps};
use crate::cmd;
use crate::stdlib::helpers;
use easy_error::{Error, bail};
use proctitle::set_title;


pub fn string_system_setproctitle_base(vm: &mut VM, op: StackOps, err_prefix: String) -> Result<&mut VM, Error> {
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
    let pt_val_get = match op {
        StackOps::FromStack => vm.stack.pull(),
        StackOps::FromWorkBench => vm.stack.pull_from_workbench(),
    };

    let pt_val = match pt_val_get {
        Some(pt_val) => pt_val,
        None => {
            bail!("{} returns NO DATA #1", &err_prefix);
        }
    };

    let pt_data = match pt_val.cast_string() {
        Ok(pt_data) => pt_data,
        Err(err) => {
            bail!("{} returned for #1: {}", &err_prefix, err);
        }
    };
    set_title(pt_data);
    Ok(vm)
}


pub fn stdlib_system_setproctitle_stack(vm: &mut VM) -> Result<&mut VM, Error> {
    string_system_setproctitle_base(vm, StackOps::FromStack, "SYSTEM.SETPROCTITLE".to_string())
}

pub fn stdlib_system_setproctitle_workbench(vm: &mut VM) -> Result<&mut VM, Error> {
    string_system_setproctitle_base(vm, StackOps::FromWorkBench, "SYSTEM.SETPROCTITLE.".to_string())
}

pub fn stdlib_system_setproctitle_disabled(_vm: &mut VM) -> Result<&mut VM, Error> {
    bail!("bund SYSTEM.SETPROCTITLE functions disabled with --noio");
}

pub fn init_stdlib(cli: &cmd::Cli) {
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };
    if cli.noio {
        let _ = bc.vm.register_inline("system.setproctitle".to_string(), stdlib_system_setproctitle_disabled);
        let _ = bc.vm.register_inline("system.setproctitle.".to_string(), stdlib_system_setproctitle_disabled);
    } else {
        let _ = bc.vm.register_inline("system.setproctitle".to_string(), stdlib_system_setproctitle_stack);
        let _ = bc.vm.register_inline("system.setproctitle.".to_string(), stdlib_system_setproctitle_workbench);
    }
    drop(bc);
}
