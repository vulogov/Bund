extern crate log;
use crate::cmd;
use crate::stdlib::BUND;
use rust_multistackvm::multistackvm::{VM};
use crate::stdlib::helpers;
use easy_error::{Error, bail};
use rust_dynamic::value::Value;


pub fn stdlib_sysinfo_host_hostname(vm: &mut VM) -> Result<&mut VM, Error> {
    let hostname = match sys_metrics::host::get_hostname() {
        Ok(hostname) => hostname,
        Err(err) => {
            bail!("sysinfo.hostname returns: {}", err);
        }
    };
    vm.stack.push(Value::from_string(hostname.clone()));
    Ok(vm)
}

pub fn stdlib_sysinfo_host_kernel_version(vm: &mut VM) -> Result<&mut VM, Error> {
    let kernel_version = match sys_metrics::host::get_kernel_version() {
        Ok(kernel_version) => kernel_version,
        Err(err) => {
            bail!("sysinfo.kernel_version returns: {}", err);
        }
    };
    vm.stack.push(Value::from_string(kernel_version.clone()));
    Ok(vm)
}

pub fn stdlib_sysinfo_host_os_version(vm: &mut VM) -> Result<&mut VM, Error> {
    let os_version = match sys_metrics::host::get_os_version() {
        Ok(os_version) => os_version,
        Err(err) => {
            bail!("sysinfo.os_version returns: {}", err);
        }
    };
    vm.stack.push(Value::from_string(os_version.clone()));
    Ok(vm)
}

pub fn stdlib_sysinfo_host_system(vm: &mut VM) -> Result<&mut VM, Error> {
    let host_info = match sys_metrics::host::get_host_info() {
        Ok(host_info) => host_info,
        Err(err) => {
            bail!("sysinfo.system returns: {}", err);
        }
    };
    vm.stack.push(Value::from_string(host_info.system.clone()));
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
    let _ = bc.vm.register_inline("sysinfo.hostname".to_string(), stdlib_sysinfo_host_hostname);
    let _ = bc.vm.register_inline("sysinfo.kernel_version".to_string(), stdlib_sysinfo_host_kernel_version);
    let _ = bc.vm.register_inline("sysinfo.os_version".to_string(), stdlib_sysinfo_host_os_version);
    let _ = bc.vm.register_inline("sysinfo.system".to_string(), stdlib_sysinfo_host_system);
    drop(bc);
}
