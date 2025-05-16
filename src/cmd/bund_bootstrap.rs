extern crate log;
use crate::cmd::Cli;
use crate::stdlib::BUND;
use crate::stdlib::helpers;

#[time_graph::instrument]
pub fn run(cli: &Cli) {
    log::debug!("BOOTSTRAP::run() reached");
    match &cli.bootstrap {
        Some(bootstrap) => {
            for b in bootstrap {
                log::debug!("BOOTSTRAP: {}", &b);
                let snippet = match helpers::file_helper::get_file_from_uri(b.to_string()) {
                    Some(snippet) => snippet,
                    None => {
                        helpers::print_error::print_error_from_str_plain(format!("BOOTSTRAP snippet not found: {}", &b));
                        return;
                    }
                };
                let code = format!("{}\n", &snippet);
                let mut bc = match BUND.lock() {
                    Ok(bc) => bc,
                    Err(err) => {
                        helpers::print_error::print_error_from_str_plain(format!("{}", err));
                        return;
                    }
                };
                match bc.eval(code) {
                    Ok(_) => {}
                    Err(err) => {
                        helpers::print_error::print_error_plain(err);
                    }
                }
                drop(bc);
            }
        }
        None => {}
    }
}
