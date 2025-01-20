extern crate log;
use crate::cmd;
use crate::stdlib::BUND;
use crate::stdlib::helpers;
use rust_multistackvm::stdlib::execute_types::CF;

pub mod conditional_ifthenelse;

pub fn init_stdlib(cli: &cmd::Cli) {
    let mut cf = CF.lock().unwrap();
    cf.insert("ifthenelse".to_string(), conditional_ifthenelse::conditional_run);
    drop(cf);
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };
    let _ = bc.vm.register_inline("?ifthenelse".to_string(), conditional_ifthenelse::stdlib_conditional_ifthenelse);

    drop(bc);
}
