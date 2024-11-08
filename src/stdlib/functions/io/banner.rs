extern crate log;
use crate::stdlib::BUND;
use rust_dynamic::value::Value;
use rust_multistackvm::multistackvm::{VM, StackOps};
use crate::cmd;
use crate::stdlib::helpers;
use rust_dynamic::types::STRING;
use cfonts::{ render, Options, Fonts };
use easy_error::{Error, bail};

pub fn bund_io_banner_base(vm: &mut VM, op: StackOps, err_prefix: String) -> Result<&mut VM, Error> {
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

    let data_val = match op {
        StackOps::FromStack => vm.stack.pull(),
        StackOps::FromWorkBench => vm.stack.pull_from_workbench(),
    };

    match data_val {
        Some(data_val) => {
            match data_val.conv(STRING) {
                Ok(data) => {
                    match data.cast_string() {
                        Ok(string_data) => {
                            let res = render(
                                Options {
                                    text: String::from(string_data),
                                    font: Fonts::FontTiny,
                                    ..Options::default()
                                }
                            ).text;
                            vm.stack.push(Value::from_string(res.clone()));
                        }
                        Err(err) => {
                            bail!("{} returns: NO CASTING #1: {}", &err_prefix, err);
                        }
                    }
                }
                Err(err) => {
                    bail!("{} returns: NO CONVERSION #1: {}", &err_prefix, err);
                }
            }
        }
        None => {
            bail!("{} returns: NO DATA #1", &err_prefix);
        }
    }
    Ok(vm)
}

pub fn stdlib_bund_io_banner_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    bund_io_banner_base(vm, StackOps::FromStack, "IO.BANNER".to_string())
}

pub fn stdlib_bund_io_banner_from_wrokbench_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    bund_io_banner_base(vm, StackOps::FromWorkBench, "IO.BANNER.".to_string())
}

pub fn stdlib_bund_io_banner_disabled(_vm: &mut VM) -> Result<&mut VM, Error> {
    bail!("bund IO.BANNER functions disabled with --noio");
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
        let _ = bc.vm.register_inline("io.banner".to_string(), stdlib_bund_io_banner_disabled);
        let _ = bc.vm.register_inline("io.banner.".to_string(), stdlib_bund_io_banner_disabled);
    } else {
        let _ = bc.vm.register_inline("io.banner".to_string(), stdlib_bund_io_banner_inline);
        let _ = bc.vm.register_inline("io.banner.".to_string(), stdlib_bund_io_banner_from_wrokbench_inline);
    }
    drop(bc);
}
