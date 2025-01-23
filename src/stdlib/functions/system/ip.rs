extern crate log;
use crate::stdlib::BUND;
use rust_dynamic::value::Value;
use rust_multistackvm::multistackvm::{VM};
use crate::cmd;
use crate::stdlib::helpers;
use easy_error::{Error, bail};
use local_ip_address::{local_ip, local_ipv6};


pub fn stdlib_local_ip_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    match local_ip() {
        Ok(ip_addr) => vm.stack.push(Value::from_string(ip_addr.to_string())),
        Err(err) => bail!("SYSTEM.IP returned: {}", err),
    };
    Ok(vm)
}

pub fn stdlib_local_ipv6_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    match local_ipv6() {
        Ok(ip_addr) => vm.stack.push(Value::from_string(ip_addr.to_string())),
        Err(err) => bail!("SYSTEM.IPv6 returned: {}", err),
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
    let _ = bc.vm.register_inline("system.ip".to_string(), stdlib_local_ip_inline);
    let _ = bc.vm.register_inline("system.ipv6".to_string(), stdlib_local_ipv6_inline);
    drop(bc);
}
