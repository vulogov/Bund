extern crate log;
use crate::stdlib::BUND;
use rust_multistackvm::multistackvm::{VM};
use crate::cmd;
use crate::stdlib::helpers;
use std::env::current_dir;
use rust_dynamic::value::Value;
use easy_error::{Error, bail};

pub fn stdlib_bund_fs_cwd_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    match current_dir() {
        Ok(cwd) => {
            vm.stack.push(Value::from_string(format!("{}", cwd.display())));
        }
        Err(err) => {
            bail!("FS.CWD returned: {}", err);
        }
    }
    Ok(vm)
}

pub fn stdlib_bund_fs_cwd_disabled(_vm: &mut VM) -> Result<&mut VM, Error> {
    bail!("bund FS.CWD functions disabled with --noio");
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
        let _ = bc.vm.register_inline("fs.cwd".to_string(), stdlib_bund_fs_cwd_disabled);
    } else {
        let _ = bc.vm.register_inline("fs.cwd".to_string(), stdlib_bund_fs_cwd_inline);
    }
    drop(bc);
}
