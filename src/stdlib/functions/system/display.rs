extern crate log;
use crate::stdlib::BUND;
use rust_dynamic::types::*;
use rust_multistackvm::multistackvm::{VM};
use crate::stdlib::functions::conditional::conditional_fmt;
use crate::cmd;
use crate::stdlib::helpers;
use std::io::Write;
use std::io;
use easy_error::{Error, bail};


pub fn stdlib_display_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for DISPLAY");
    }
    let value = match vm.stack.pull() {
        Some(name_val) => name_val,
        None => bail!("DISPLAY: No value discovered on the stack"),
    };
    match value.dt {
        CONDITIONAL => {
            let fmt_type_val = match value.get("type") {
                Ok(fmt_type_val) => fmt_type_val,
                Err(err) => bail!("DISPLAY: getting fmt type returns error: {}", err),
            };
            let fmt_type = match fmt_type_val.cast_string() {
                Ok(fmt_type) => fmt_type,
                Err(err) => bail!("DISPLAY: casting fmt type returns error: {}", err),
            };
            match fmt_type.as_str() {
                "fmt" => {
                    log::debug!("Running fmt conditional");
                    match conditional_fmt::conditional_run(vm, value) {
                        Ok(_) => {
                            let out_value = match vm.stack.pull() {
                                Some(out_val) => out_val,
                                None => bail!("DISPLAY: No value discovered on the stack"),
                            };
                            match out_value.cast_string() {
                                Ok(str_val) => {
                                    println!("{}", &str_val);
                                }
                                Err(err) => bail!("DISPLAY: casting out value returns error: {}", err),
                            };
                        }
                        Err(err) => {
                            bail!("DISPLAY: conditional returns error: {}", err);
                        }
                    }
                }
                _ => {
                    bail!("FMT.STR: conditional is of incorrect type: {}", &fmt_type);
                }
            }
        }
        _ => {
            log::debug!("Running Value::conv(STRING)");
            match value.conv(STRING) {
                Ok(str_value) => {
                    println!("{}", str_value.cast_string().unwrap());
                }
                Err(err) => {
                    bail!("FMT.STR: conversion to STRING returned error: {}", err);
                }
            }
        }
    }
    let _ = io::stdout().flush();
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
    let _ = bc.vm.register_inline("display".to_string(), stdlib_display_inline);
    drop(bc);
}
