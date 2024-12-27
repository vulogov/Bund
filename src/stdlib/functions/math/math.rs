use rust_multistackvm::multistackvm::{VM};
use crate::stdlib::BUND;
use crate::stdlib::helpers;
use rust_dynamic::value::Value;
use easy_error::{Error, bail};
use crate::cmd;
use mathlab::math;

pub enum Ops {
    Csc,
}

pub fn stdlib_float_csc_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_math_float_inline(vm, Ops::Csc)
}



pub fn stdlib_math_float_inline(vm: &mut VM, op: Ops) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline float_op");
    }
    match vm.stack.pull() {
        Some(value) => {
            match value.cast_float() {
                Ok(fvalue) => {
                    match op {
                        Ops::Csc => {
                            vm.stack.push(Value::from_float(math::csc(fvalue)));
                        }
                    }
                }
                Err(err) => {
                    bail!("FLOAT_OP returns error: {}", err);
                }
            }
        }
        None => {
            bail!("FLOAT_OP returns: NO DATA #1");
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
    let _ = bc.vm.register_inline("math.cosecant".to_string(), stdlib_float_csc_inline);
}
