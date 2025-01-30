extern crate log;
use crate::stdlib::BUND;
use rust_dynamic::value::Value;
use rust_multistackvm::multistackvm::{VM};
use crate::cmd;
use crate::stdlib::helpers;
use easy_error::{Error, bail};
use sys_locale;


pub fn stdlib_locale_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    match sys_locale::get_locale() {
        Some(loc) => vm.stack.push(Value::from_string(loc)),
        None => bail!("SYSTEM.LOCALE can not be found"),
    };
    Ok(vm)
}

pub fn init_stdlib(cli: &cmd::Cli) {
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };
    let _ = bc.vm.register_inline("system.locale".to_string(), stdlib_locale_inline);
    drop(bc);
}
