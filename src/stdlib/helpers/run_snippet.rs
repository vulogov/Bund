extern crate log;

use crate::cmd;
use crate::stdlib::helpers;
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
    match bc.run(code) {
        Ok(val) => {
            match val {
                Some(returned_value) => {
                    println!("{}", &returned_value);
                }
                None => {
                    log::warn!("Snippet returned no value");
                }
            }
        }
        Err(err) => {
            helpers::print_error::print_error(err, cli);
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
    match bc.eval(code) {
        Ok(_) => {}
        Err(err) => {
            helpers::print_error::print_error(err, cli);
        }
    }
    drop(bc);
}

pub fn run_snippet(snippet: String) {
    log::debug!("Running snippet: {}", &snippet);
    let code = format!("{}", &snippet);
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
