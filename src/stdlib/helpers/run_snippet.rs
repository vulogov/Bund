
extern crate log;

use crate::cmd;
use crate::stdlib::helpers;
use crate::stdlib::functions::debug_fun;
use std::ops::DerefMut;
use crate::stdlib::BUND;


pub fn run_snippet_for_cmd(snippet: String, cli: &cmd::Cli) {
    let code = format!("{}", &snippet);
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };
    if cli.debugger {
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
                helpers::print_error::print_error(err, cli);
                if cli.debug_shell {
                    let _ = debug_fun::debug_shell::debug_shell(&mut bc.deref_mut().vm);
                }
            }
        }
    }
    drop(bc);
}

pub fn run_snippet_for_script(snippet: String, cli: &cmd::Cli) {
    let code = format!("{}", &snippet);
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };
    if cli.debugger {
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
        match bc.eval(code) {
            Ok(_) => {}
            Err(err) => {
                helpers::print_error::print_error(err, cli);
                if cli.debug_shell {
                    let _ = debug_fun::debug_shell::debug_shell(&mut bc.deref_mut().vm);
                }
            }
        }
    }
    drop(bc);
}

pub fn run_snippet(snippet: String) {
    log::debug!("Running snippet: {}", &snippet);
    let code = format!("{}\n", &snippet);
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str_plain(format!("{}", err));
            return;
        }
    };
    log::debug!("Reached bc::eval()");
    match bc.eval(code) {
        Ok(_) => {}
        Err(err) => {
            helpers::print_error::print_error_plain(err);
        }
    }
    drop(bc);
}
