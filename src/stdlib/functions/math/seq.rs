use rust_multistackvm::multistackvm::{VM};
use crate::stdlib::BUND;
use crate::stdlib::helpers;
use rust_dynamic::value::Value;
use easy_error::{Error, bail};
use crate::cmd;
use mathlab::functions::args;

pub enum Ops {
    Exp,
}

pub enum SeqOrd {
    Asc,
    Desc,
}

pub fn stdlib_float_gen_seq_asc_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_math_float_gen_seq_inline(vm, SeqOrd::Asc)
}

pub fn stdlib_float_gen_seq_desc_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_math_float_gen_seq_inline(vm, SeqOrd::Desc)
}


pub fn stdlib_math_float_gen_seq_inline(vm: &mut VM, ord: SeqOrd) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 3 {
        bail!("Stack is too shallow for inline SEQ");
    }
    match vm.stack.pull() {
        Some(x_value) => {
            match x_value.cast_float() {
                Ok(xvalue) => {
                    match vm.stack.pull() {
                        Some(s_value) => {
                            match s_value.cast_float() {
                                Ok(step) => {
                                    match vm.stack.pull() {
                                        Some(n_value) => {
                                            match n_value.cast_int() {
                                                Ok(n) => {
                                                    let fres = match ord {
                                                        SeqOrd::Asc => args::range(xvalue, step, n as usize, "asc"),
                                                        SeqOrd::Desc => args::range(xvalue, step, n as usize, "desc"),
                                                    };
                                                    let mut res: Vec<Value> = Vec::new();
                                                    for v in fres {
                                                        res.push(Value::from_float(v));
                                                    }
                                                    vm.stack.push(Value::from_list(res));
                                                }
                                                Err(err) => {
                                                    bail!("Casting N returns: {}", err);
                                                }
                                            }
                                        }
                                        None => {
                                            bail!("SEQ_OP returns: NO DATA #3");
                                        }
                                    }
                                }
                                Err(err) => {
                                    bail!("Casting Step returns: {}", err);
                                }
                            }
                        }
                        None => {
                            bail!("SEQ_OP returns: NO DATA #2");
                        }
                    }
                }
                Err(err) => {
                    bail!("Casting X returns: {}", err);
                }
            }
        }
        None => {
            bail!("SEQ_OP returns: NO DATA #1");
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
    let _ = bc.vm.register_inline("seq.asc".to_string(), stdlib_float_gen_seq_asc_inline);
    let _ = bc.vm.register_inline("seq.desc".to_string(), stdlib_float_gen_seq_desc_inline);
}
