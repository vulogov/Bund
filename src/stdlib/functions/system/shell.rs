extern crate log;
use crate::stdlib::BUND;
use rust_dynamic::value::Value;
use rust_multistackvm::multistackvm::{VM, StackOps};
use crate::cmd;
use crate::stdlib::helpers;
use easy_error::{Error, bail};
use duct_sh;


pub fn string_system_shell_base(vm: &mut VM, op: StackOps, err_prefix: String) -> Result<&mut VM, Error> {
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
    let cmd_val_get = match op {
        StackOps::FromStack => vm.stack.pull(),
        StackOps::FromWorkBench => vm.stack.pull_from_workbench(),
    };

    let cmd_val = match cmd_val_get {
        Some(cmd_val) => cmd_val,
        None => {
            bail!("{} returns NO DATA #1", &err_prefix);
        }
    };

    let cmd_data = match cmd_val.cast_string() {
        Ok(cmd_data) => cmd_data,
        Err(err) => {
            bail!("{} returned for #1: {}", &err_prefix, err);
        }
    };
    match duct_sh::sh_dangerous(cmd_data).read() {
        Ok(res) => {
            match op {
                StackOps::FromStack => vm.stack.push(Value::from_string(res)),
                StackOps::FromWorkBench => vm.stack.push_to_workbench(Value::from_string(res)),
            };
        }
        Err(err) => {
            bail!("{} returns: {}", &err_prefix, err);
        }
    }
    Ok(vm)
}


pub fn stdlib_system_shell_stack(vm: &mut VM) -> Result<&mut VM, Error> {
    string_system_shell_base(vm, StackOps::FromStack, "SYSTEM.SHELL".to_string())
}

pub fn stdlib_system_shell_workbench(vm: &mut VM) -> Result<&mut VM, Error> {
    string_system_shell_base(vm, StackOps::FromWorkBench, "SYSTEM.SHELL.".to_string())
}

pub fn stdlib_system_shell_disabled(_vm: &mut VM) -> Result<&mut VM, Error> {
    bail!("bund SYSTEM.SHELL functions disabled with --noio");
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
        let _ = bc.vm.register_inline("system.shell".to_string(), stdlib_system_shell_disabled);
        let _ = bc.vm.register_inline("system.shell.".to_string(), stdlib_system_shell_disabled);
    } else {
        let _ = bc.vm.register_inline("system.shell".to_string(), stdlib_system_shell_stack);
        let _ = bc.vm.register_inline("system.shell.".to_string(), stdlib_system_shell_workbench);
    }
    drop(bc);
}
