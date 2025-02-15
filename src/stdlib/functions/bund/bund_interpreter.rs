extern crate log;
use crate::cmd;
use crate::stdlib::BUND;
use crate::stdlib::helpers;
use rust_dynamic::value::Value;
use rust_multistackvm::multistackvm::{VM};
use rust_dynamic::types::*;
use bund_language_parser::bund_parse;
use easy_error::{Error, bail};


#[time_graph::instrument]
pub fn stdlib_bund_compile(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for BUND.COMPILE");
    }
    let src_value = match vm.stack.pull() {
        Some(src_value) => src_value,
        None => {
            bail!("BUND.COMPILE returns NO DATA #1");
        }
    };
    let code = match src_value.cast_string() {
        Ok(code) => code,
        Err(err) => {
            bail!("BUND.COMPILE casting string returns: {}", err);
        }
    };
    let source = format!("{}\n", code.clone());
    match bund_parse(&source) {
        Ok(words) => {
            let mut res = Value::list();
            for word in words {
                match word.dt {
                    EXIT => {
                        break;
                    }
                    _ => {
                        res = res.push(word);
                    }
                }
            }
            vm.stack.push(res);
        }
        Err(err) => {
            bail!("BUND.COMPILE error in parsing of BUND code: {}", err);
        }
    }
    Ok(vm)
}

#[time_graph::instrument]
pub fn stdlib_bund_apply(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for APPLY");
    }
    let value = match vm.stack.pull() {
        Some(value) => value,
        None => {
            bail!("APPLY returns NO DATA #1");
        }
    };
    vm.apply(value)
}



pub fn init_stdlib(cli: &cmd::Cli) {
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };
    let _ = bc.vm.register_inline("compile".to_string(), stdlib_bund_compile);
    let _ = bc.vm.register_inline("apply".to_string(), stdlib_bund_apply);
    drop(bc);
}
