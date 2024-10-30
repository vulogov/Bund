
extern crate log;
use crate::cmd;
use crate::stdlib::BUND;
use crate::stdlib::helpers;
use rust_multistackvm::multistackvm::{VM};
use easy_error::{Error, bail};

fn bund_save(vm: &mut VM, _fie_name: String) -> Result<&mut VM, Error> {
    Ok(vm)
}

pub fn stdlib_bund_save(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for SAVE");
    }
    let file_name_value = match vm.stack.pull() {
        Some(file_name_value) => file_name_value,
        None => {
            bail!("SAVE returns NO DATA #1");
        }
    };
    let file_name = match file_name_value.cast_string() {
        Ok(file_name) => file_name,
        Err(err) => {
            bail!("SAVE casting string returns: {}", err);
        }
    };
    bund_save(vm, file_name)
}

pub fn stdlib_bund_save_disabled(_vm: &mut VM) -> Result<&mut VM, Error> {
    bail!("bund SAVE functions disabled with --noio");
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
        let _ = bc.vm.register_inline("save".to_string(), stdlib_bund_save_disabled);
    } else {
        let _ = bc.vm.register_inline("save".to_string(), stdlib_bund_save);
    }
    drop(bc);
}
