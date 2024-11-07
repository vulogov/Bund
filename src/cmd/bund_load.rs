extern crate log;
use crate::cmd;
use crate::stdlib::BUND;
use crate::stdlib::helpers;
use rust_dynamic::value::Value;
use crate::stdlib::functions::{bund};

pub fn run(cli: &cmd::Cli, load_arg: &cmd::Load) {
    log::debug!("LOAD::run() reached");

    match bund::bund_args::ARGS.lock() {
        Ok(mut args) => {
            for a in &load_arg.args {
                let _ = args.push_inplace(Value::from_string(a.clone()));
            }
            drop(args);
        }
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    }

    let mut conn = match helpers::world::open(load_arg.world.clone()) {
        Ok(conn) => conn,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };
    let res = bund::bund_load::bund_load_with_scripts(&mut bc.vm, &mut conn);
    match res {
        Ok(_) => {},
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
        }
    }
    drop(bc);
    match conn.close() {
        Ok(_) => {},
        Err(err) => {
            log::debug!("Closing connection to the world returns error: {:?}", err);
        }
    }

}
