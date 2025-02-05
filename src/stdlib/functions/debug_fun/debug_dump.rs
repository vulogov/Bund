extern crate log;
use crate::cmd;
use rust_dynamic::types::*;
use crate::stdlib::BUND;
use rust_multistackvm::multistackvm::{VM};
use crate::stdlib::helpers;
use easy_error::{Error, bail};
use bytemuck::bytes_of;
use hexdump;



pub fn stdlib_debug_dump(vm: &mut VM) -> Result<&mut VM, Error> {
    let value_val = match vm.stack.peek() {
        Some(value_val) => value_val,
        None => bail!("DUMP: NO DATA #1"),
    };
    match value_val.dt {
        INTEGER => {
            let int_raw = match value_val.cast_int() {
                Ok(int_raw) => int_raw,
                Err(err) => bail!("DUMP: error CASTING to INT: {}", err),
            };
            hexdump::hexdump(bytes_of(&int_raw));
        }
        FLOAT => {
            let float_raw = match value_val.cast_float() {
                Ok(float_raw) => float_raw,
                Err(err) => bail!("DUMP: error CASTING to FLOAT: {}", err),
            };
            hexdump::hexdump(bytes_of(&float_raw));
        }
        BOOL => {
            let bool_raw = match value_val.cast_bool() {
                Ok(bool_raw) => bool_raw,
                Err(err) => bail!("DUMP: error CASTING to BOOL: {}", err),
            };
            hexdump::hexdump(bytes_of(&bool_raw));
        }
        STRING => {
            let str_val = match value_val.conv(STRING) {
                Ok(str_val) => str_val,
                Err(err) => bail!("DUMP: error converting to STRING: {}", err),
            };
            let str_raw = match str_val.cast_string() {
                Ok(str_raw) => str_raw,
                Err(err) => bail!("DUMP: error CASTING to STRING: {}", err),
            };
            hexdump::hexdump(str_raw.as_bytes());
        }
        _ => {
            match value_val.to_binary() {
                Ok(bin_data) => {
                    hexdump::hexdump(&bin_data);
                }
                Err(err) => {
                    bail!("DUMP: error converting to binary: {}", err);
                }
            }
        }
    }
    Ok(vm)
}


pub fn init_stdlib(cli: &cmd::Cli) {
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };
    let _ = bc.vm.register_inline("debug.dump".to_string(), stdlib_debug_dump);
    drop(bc);
}
