
extern crate log;

use crate::cmd;
use crate::stdlib::helpers;
use crate::stdlib::functions::debug_fun;
use std::ops::DerefMut;
use crate::stdlib::BUND;
use rust_dynamic::value::Value;

use easy_error::{Error, bail};

#[time_graph::instrument]
pub fn run_snippet_for_cmd(snippet: String, cli: &cmd::Cli) {
    let code = format!("{}\n", &snippet);
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };
    if cli.debugger {
        log::warn!("Dropping into DEBUGGER");
        match debug_fun::debug_debug::bund_debugger(&mut bc.vm, snippet) {
            Ok(_) => {

            }
            Err(err) => {
                helpers::print_error::print_error(err, cli);
                if cli.debug_shell {
                    let _ = debug_fun::debug_shell::debug_shell(&mut bc.deref_mut().vm);
                }
            }
        }
    } else {
        log::debug!("Execute code");
        match bc.run(code) {
            Ok(val) => {
                match val {
                    Some(returned_value) => {
                        println!("{}", &returned_value);
                    }
                    None => {
                        log::debug!("Snippet returned no value");
                    }
                }
            }
            Err(err) => {
                helpers::print_error::print_error_with_vm(&mut bc.vm, err, cli);
                if cli.debug_shell {
                    let _ = debug_fun::debug_shell::debug_shell(&mut bc.deref_mut().vm);
                }
            }
        }
    }
    drop(bc);
}

#[time_graph::instrument]
pub fn run_snippet_for_script(snippet: String, cli: &cmd::Cli) {
    let code = format!("{}\n", &snippet);
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };
    if cli.debugger {
        log::warn!("Dropping into DEBUGGER");
        match debug_fun::debug_debug::bund_debugger(&mut bc.vm, snippet) {
            Ok(_) => {

            }
            Err(err) => {
                helpers::print_error::print_error(err, cli);
                if cli.debug_shell {
                    let _ = debug_fun::debug_shell::debug_shell(&mut bc.deref_mut().vm);
                }
            }
        }
    } else {
        log::debug!("Execute code");
        match bc.eval(code) {
            Ok(_) => {}
            Err(err) => {
                helpers::print_error::print_error_with_vm(&mut bc.vm, err, cli);
                if cli.debug_shell {
                    let _ = debug_fun::debug_shell::debug_shell(&mut bc.deref_mut().vm);
                }
            }
        }
    }
    drop(bc);
}

#[time_graph::instrument]
pub fn run_snippet_and_return_value(snippet: String) -> Result<(Value, Option<Value>, usize), Error> {
    let code = format!("{}\n", &snippet);
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => bail!("Locking interpreter returns: {}", err),
    };

    log::debug!("Execute code");
    match bc.eval(code) {
        Ok(_) => {}
        Err(err) => bail!("BUND script exit with: {}", err),
    };
    if bc.vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for returning a value");
    } else {
        match bc.vm.stack.pull() {
            Some(value) => {
                let is_desc = bc.vm.stack.pull();
                let stack_len = bc.vm.stack.current_stack_len();
                if stack_len > 0 {
                    let _ = bc.eval("debug.display_stack");
                }
                bc.vm.stack.clear();
                drop(bc);
                // println!("SSS {:?}: {:?} {:?} {}",  &snippet, &value, &is_desc, &stack_len);
                return Ok((value, is_desc, stack_len));
            }
            None => {
                drop(bc);
                bail!("Can not obtain value for return operation");
            }
        }
    }
}


#[time_graph::instrument]
pub fn run_snippet(snippet: String) {
    log::debug!("Running snippet: {}", &snippet);
    let code = format!("{}\n", &snippet);
    let the_arg = cmd::CLI.lock().unwrap();
    let cli = the_arg.clone();
    drop(the_arg);
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), &cli);
            return;
        }
    };
    log::debug!("Reached bc::eval()");
    match bc.eval(code) {
        Ok(_) => {}
        Err(err) => {
            helpers::print_error::print_error_with_vm(&mut bc.vm, err, &cli);
        }
    }
    drop(bc);
}
