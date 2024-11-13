extern crate log;
use crate::cmd;
use crate::stdlib::BUND;
use rust_multistackvm::multistackvm::{VM};
use crate::stdlib::helpers;
use easy_error::{Error};
use rust_dynamic::value::Value;

pub fn bund_sysinfo_virtualization() -> String {
    return format!("{:?}", sys_metrics::virt::get_virt_info());
}

pub fn stdlib_sysinfo_virtualization(vm: &mut VM) -> Result<&mut VM, Error> {
    vm.stack.push(Value::from_string(bund_sysinfo_virtualization().clone()));
    Ok(vm)
}
pub fn stdlib_sysinfo_is_virtualization(vm: &mut VM) -> Result<&mut VM, Error> {
    match sys_metrics::virt::get_virt_info() {
        sys_metrics::virt::Virtualization::Unknown => {
            vm.stack.push(Value::from_bool(false));
        }
        _ => {
            vm.stack.push(Value::from_bool(true));
        }
    }
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
    let _ = bc.vm.register_inline("sysinfo.virtualization".to_string(), stdlib_sysinfo_virtualization);
    let _ = bc.vm.register_inline("sysinfo.virtualization?".to_string(), stdlib_sysinfo_is_virtualization);
    drop(bc);
}
