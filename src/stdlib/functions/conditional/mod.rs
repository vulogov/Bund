extern crate log;
use crate::cmd;
use crate::stdlib::BUND;
use crate::stdlib::helpers;
use rust_multistackvm::stdlib::execute_types::CF;

pub mod raise;

pub mod conditional_ifthenelse;
pub mod conditional_tryexcept;

pub fn init_stdlib(cli: &cmd::Cli) {
    let mut cf = CF.lock().unwrap();
    cf.insert("ifthenelse".to_string(), conditional_ifthenelse::conditional_run);
    cf.insert("tryexcept".to_string(), conditional_tryexcept::conditional_run);
    drop(cf);
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };
    let _ = bc.vm.register_inline("?ifthenelse".to_string(), conditional_ifthenelse::stdlib_conditional_ifthenelse);
    let _ = bc.vm.register_inline("?try".to_string(), conditional_tryexcept::stdlib_conditional_tryexcept);
    let _ = bc.vm.register_inline("raise".to_string(), raise::stdlib_conditional_raise);

    drop(bc);
}
