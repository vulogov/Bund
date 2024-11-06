
extern crate log;
use crate::cmd;
use crate::stdlib::BUND;
use crate::stdlib::helpers;
use rust_multistackvm::multistackvm::{VM, StackOps};
use easy_error::{Error, bail};


pub fn bund_use_base(vm: &mut VM, op: StackOps, err_prefix: String) -> Result<&mut VM, Error> {
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
    let snippet_addr_val = match op {
        StackOps::FromStack => vm.stack.pull(),
        StackOps::FromWorkBench => vm.stack.pull_from_workbench(),
    };
    match snippet_addr_val {
        Some(snippet_val) => {
            match snippet_val.cast_string() {
                Ok(snippet_addr) => {
                    match helpers::file_helper::get_file_from_uri(snippet_addr.clone()) {
                        Some(snippet) => {
                            return helpers::eval::bund_compile_and_eval(vm, snippet);
                        }
                        None => {
                            bail!("{} can not get from {}", &err_prefix, &snippet_addr);
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
}


pub fn stdlib_bund_use_from_stack_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    bund_use_base(vm, StackOps::FromStack, "USE".to_string())
}

pub fn stdlib_bund_use_from_workbech_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    bund_use_base(vm, StackOps::FromWorkBench, "USE.".to_string())
}

pub fn stdlib_bund_use_disabled(_vm: &mut VM) -> Result<&mut VM, Error> {
    bail!("bund USE functions disabled with --noeval");
}

pub fn init_stdlib(cli: &cmd::Cli) {
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };
    if cli.noeval {
        let _ = bc.vm.register_inline("use".to_string(), stdlib_bund_use_disabled);
        let _ = bc.vm.register_inline("use.".to_string(), stdlib_bund_use_disabled);
    } else {
        let _ = bc.vm.register_inline("use".to_string(), stdlib_bund_use_from_stack_inline);
        let _ = bc.vm.register_inline("use.".to_string(), stdlib_bund_use_from_workbech_inline);
    }
    drop(bc);
}
