extern crate log;
use crate::stdlib::BUND;
use rust_multistackvm::multistackvm::{VM, StackOps};
use rust_dynamic::value::Value;
use crate::cmd;
use crate::stdlib::helpers;
use easy_error::{Error, bail};

#[derive(Debug, Clone)]
pub enum FsysOperations {
    IsFile
}

pub fn bund_filesystem_base(vm: &mut VM, op: StackOps, fop: FsysOperations, err_prefix: String) -> Result<&mut VM, Error> {
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
    let fn_val = match op {
        StackOps::FromStack => vm.stack.pull(),
        StackOps::FromWorkBench => vm.stack.pull_from_workbench(),
    };
    match fn_val {
        Some(fn_val) => {
            match fn_val.cast_string() {
                Ok(fn_name) => {
                    let _ = match fop {
                        FsysOperations::IsFile => vm.stack.push(Value::from_bool(helpers::filesystem_helper::filesystem_is_file(fn_name))),
                    };
                }
                Err(err) => {
                    bail!("{} returns: {}", &err_prefix, err);
                }
            }
        }
        None => {
            bail!("{} returns: NO DATA", &err_prefix);
        }
    }
    Ok(vm)
}

pub fn stdlib_fs_is_file_from_stack_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    bund_filesystem_base(vm, StackOps::FromStack, FsysOperations::IsFile, "FS.IS_FILE".to_string())
}
pub fn stdlib_fs_is_file_from_wb_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    bund_filesystem_base(vm, StackOps::FromStack, FsysOperations::IsFile, "FS.IS_FILE".to_string())
}

pub fn stdlib_bund_filesystem_disabled(_vm: &mut VM) -> Result<&mut VM, Error> {
    bail!("bund FILESYSTEM functions disabled with --noio");
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
        let _ = bc.vm.register_inline("fs.is_file".to_string(), stdlib_bund_filesystem_disabled);
        let _ = bc.vm.register_inline("fs.is_file.".to_string(), stdlib_bund_filesystem_disabled);
    } else {
        let _ = bc.vm.register_inline("fs.is_file".to_string(), stdlib_fs_is_file_from_stack_inline);
        let _ = bc.vm.register_inline("fs_is_file.".to_string(), stdlib_fs_is_file_from_wb_inline);
    }
    drop(bc);
}
