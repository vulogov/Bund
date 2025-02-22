extern crate log;
use crate::cmd;
use crate::stdlib::BUND;
use rust_multistackvm::multistackvm::{VM};
use crate::stdlib::helpers;
use easy_error::{Error, bail};

#[derive(Debug, Clone)]
pub enum TraceLvl {
    Info,
    Warn,
    Error,
    Debug,
    Trace,
}

pub fn stdlib_trace_base(vm: &mut VM, lvl: TraceLvl, err_prefix: String) -> Result<&mut VM, Error> {
    let value_val = match vm.stack.pull() {
        Some(value_val) => value_val,
        None => bail!("{}: NO DATA #1", &err_prefix),
    };
    match value_val.cast_string() {
        Ok(value_str) => {
            match lvl {
                TraceLvl::Info => log::info!("{}",value_str),
                TraceLvl::Warn => log::warn!("{}",value_str),
                TraceLvl::Error => log::error!("{}",value_str),
                TraceLvl::Debug => log::debug!("{}",value_str),
                TraceLvl::Trace => log::trace!("{}",value_str),
            }
        }
        Err(err) => bail!("{}: Error casting tracing message: {}", &err_prefix, err),
    }
    Ok(vm)
}

pub fn stdlib_trace_info(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_trace_base(vm, TraceLvl::Info, "LOG.INFO".to_string())
}
pub fn stdlib_trace_warn(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_trace_base(vm, TraceLvl::Warn, "LOG.WARNING".to_string())
}
pub fn stdlib_trace_error(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_trace_base(vm, TraceLvl::Error, "LOG.ERROR".to_string())
}
pub fn stdlib_trace_debug(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_trace_base(vm, TraceLvl::Debug, "LOG.DEBUG".to_string())
}
pub fn stdlib_trace_trace(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_trace_base(vm, TraceLvl::Trace, "LOG.TRACE".to_string())
}

pub fn init_stdlib(cli: &cmd::Cli) {
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };
    let _ = bc.vm.register_inline("log.info".to_string(), stdlib_trace_info);
    let _ = bc.vm.register_inline("log.warning".to_string(), stdlib_trace_warn);
    let _ = bc.vm.register_inline("log.error".to_string(), stdlib_trace_error);
    let _ = bc.vm.register_inline("log.debug".to_string(), stdlib_trace_debug);
    let _ = bc.vm.register_inline("log.trace".to_string(), stdlib_trace_trace);
    drop(bc);
}
