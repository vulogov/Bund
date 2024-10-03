extern crate log;
use crate::stdlib::BUND;
use rust_multistackvm::multistackvm::{VM, StackOps};
use rust_dynamic::value::Value;
use crate::cmd;
use crate::stdlib::helpers;
use easy_error::{Error, bail};

#[derive(Debug, Clone)]
pub enum DataSource {
    File,
    Url,
}

pub fn bund_file_base(vm: &mut VM, op: StackOps, src: DataSource, err_prefix: String) -> Result<&mut VM, Error> {
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
                    let data = match src {
                        DataSource::File => helpers::file_helper::get_file_from_file(fn_name),
                        DataSource::Url => helpers::file_helper::get_file_from_uri(fn_name),
                    };
                    match data {
                        Some(data) => {
                            match op {
                                StackOps::FromStack => vm.stack.push(Value::from_string(data)),
                                StackOps::FromWorkBench => vm.stack.push_to_workbench(Value::from_string(data)),
                            };
                        }
                        None => {
                            bail!("{} gets no data", &err_prefix);
                        }
                    }
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

pub fn stdlib_file_from_stack_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    bund_file_base(vm, StackOps::FromStack, DataSource::File, "FILE".to_string())
}

pub fn stdlib_file_from_workbech_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    bund_file_base(vm, StackOps::FromWorkBench, DataSource::File, "FILE.".to_string())
}

pub fn stdlib_url_from_stack_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    bund_file_base(vm, StackOps::FromStack, DataSource::Url, "URL".to_string())
}

pub fn stdlib_url_from_workbech_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    bund_file_base(vm, StackOps::FromWorkBench, DataSource::Url, "URL.".to_string())
}

pub fn stdlib_bund_file_disabled(_vm: &mut VM) -> Result<&mut VM, Error> {
    bail!("bund FILE functions disabled with --noio");
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
        let _ = bc.vm.register_inline("file".to_string(), stdlib_bund_file_disabled);
        let _ = bc.vm.register_inline("file.".to_string(), stdlib_bund_file_disabled);
        let _ = bc.vm.register_inline("stdin".to_string(), stdlib_bund_file_disabled);
        let _ = bc.vm.register_inline("stdin.".to_string(), stdlib_bund_file_disabled);
        let _ = bc.vm.register_inline("url".to_string(), stdlib_bund_file_disabled);
        let _ = bc.vm.register_inline("url.".to_string(), stdlib_bund_file_disabled);
    } else {
        let _ = bc.vm.register_inline("file".to_string(), stdlib_file_from_stack_inline);
        let _ = bc.vm.register_inline("file.".to_string(), stdlib_file_from_workbech_inline);
        let _ = bc.vm.register_inline("url".to_string(), stdlib_url_from_stack_inline);
        let _ = bc.vm.register_inline("url.".to_string(), stdlib_url_from_workbech_inline);
    }
    drop(bc);
}
