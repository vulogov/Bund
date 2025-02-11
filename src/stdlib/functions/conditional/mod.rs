extern crate log;
use crate::cmd;
use crate::stdlib::BUND;
use crate::stdlib::helpers;
use rust_multistackvm::stdlib::execute_types::CF;

pub mod raise;

pub mod conditional_ifthenelse;
pub mod conditional_tryexcept;
pub mod conditional_error;
pub mod conditional_ctx;
pub mod conditional_curry;
pub mod conditional_fmt;
pub mod conditional_csv;
pub mod conditional_sqlite;

pub fn init_stdlib(cli: &cmd::Cli) {
    let mut cf = CF.lock().unwrap();
    cf.insert("ifthenelse".to_string(), conditional_ifthenelse::conditional_run);
    cf.insert("tryexcept".to_string(), conditional_tryexcept::conditional_run);
    cf.insert("error".to_string(), conditional_error::conditional_run);
    cf.insert("context".to_string(), conditional_ctx::conditional_run);
    cf.insert("curry".to_string(), conditional_curry::conditional_run);
    cf.insert("fmt".to_string(), conditional_fmt::conditional_run);
    cf.insert("csv".to_string(), conditional_csv::conditional_run);
    cf.insert("sqlite".to_string(), conditional_sqlite::conditional_run);
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
    let _ = bc.vm.register_inline("?error".to_string(), conditional_error::stdlib_conditional_error);
    let _ = bc.vm.register_inline("context".to_string(), conditional_ctx::stdlib_conditional_ctx);
    let _ = bc.vm.register_inline("curry".to_string(), conditional_curry::stdlib_conditional_curry);
    let _ = bc.vm.register_inline("fmt".to_string(), conditional_fmt::stdlib_conditional_fmt);
    let _ = bc.vm.register_inline("csv".to_string(), conditional_csv::stdlib_conditional_csv);
    let _ = bc.vm.register_inline("sqlite".to_string(), conditional_sqlite::stdlib_conditional_sqlite);
    let _ = bc.vm.register_inline("raise".to_string(), raise::stdlib_conditional_raise);

    drop(bc);
}
