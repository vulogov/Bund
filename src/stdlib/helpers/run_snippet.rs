
extern crate log;

use crate::cmd;
use crate::stdlib::helpers;
use crate::stdlib::functions::debug_fun;
use std::ops::DerefMut;
use crate::stdlib::BUND;

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
