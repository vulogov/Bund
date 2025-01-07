extern crate log;
use crate::stdlib::BUND;
use rust_dynamic::value::Value;
use rust_multistackvm::multistackvm::{VM, StackOps};
use crate::cmd;
use crate::stdlib::helpers;
use easy_error::{Error, bail};
use easy_reader::EasyReader;
use std::fs::File;


pub fn string_io_textfile_base(vm: &mut VM, op: StackOps, err_prefix: String) -> Result<&mut VM, Error> {
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
    let fname_val_get = match op {
        StackOps::FromStack => vm.stack.pull(),
        StackOps::FromWorkBench => vm.stack.pull_from_workbench(),
    };

    let fname_val = match fname_val_get {
        Some(fname_val) => fname_val,
        None => {
            bail!("{} returns NO DATA #1", &err_prefix);
        }
    };

    let fname = match fname_val.cast_string() {
        Ok(fname) => fname,
        Err(err) => {
            bail!("{} returned for #1: {}", &err_prefix, err);
        }
    };
    match File::open(fname) {
        Ok(file) => {
            let mut res = Value::list();
            match EasyReader::new(file) {
                Ok(mut reader) => {
                    let _ = reader.build_index();
                    reader.bof();
                    loop {
                        match reader.next_line() {
                            Ok(Some(line)) => {
                                res = res.push(Value::from_string(line));
                            }
                            Ok(None) => break,
                            _ => break,
                        }
                    }
                }
                Err(err) => {
                    bail!("{} returns error: {}", &err_prefix, err);
                }
            }
            match op {
                StackOps::FromStack => vm.stack.push(res),
                StackOps::FromWorkBench => vm.stack.push_to_workbench(res),
            };
        }
        Err(err) => {
            bail!("{} returns: {}", &err_prefix, err);
        }
    }
    Ok(vm)
}


pub fn stdlib_io_textfile_stack(vm: &mut VM) -> Result<&mut VM, Error> {
    string_io_textfile_base(vm, StackOps::FromStack, "IO.TEXTFILE".to_string())
}

pub fn stdlib_io_textfile_workbench(vm: &mut VM) -> Result<&mut VM, Error> {
    string_io_textfile_base(vm, StackOps::FromWorkBench, "IO.TEXTFILE.".to_string())
}

pub fn stdlib_io_textfile_disabled(_vm: &mut VM) -> Result<&mut VM, Error> {
    bail!("bund IO.TEXTFILE functions disabled with --noio");
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
        let _ = bc.vm.register_inline("io.textfile".to_string(), stdlib_io_textfile_disabled);
        let _ = bc.vm.register_inline("io.textfile.".to_string(), stdlib_io_textfile_disabled);
    } else {
        let _ = bc.vm.register_inline("io.textfile".to_string(), stdlib_io_textfile_stack);
        let _ = bc.vm.register_inline("io.textfile.".to_string(), stdlib_io_textfile_workbench);
    }
    drop(bc);
}
