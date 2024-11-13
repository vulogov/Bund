extern crate log;
use crate::stdlib::BUND;
//use rust_dynamic::value::Value;
use rust_multistackvm::multistackvm::{VM, StackOps};
use crate::cmd;
use crate::stdlib::helpers;
use easy_error::{Error, bail};

#[derive(Debug, Clone)]
pub enum CrossbusOperations {
    Send,
    Recv,
}


fn stdlib_bund_crossbus_base(vm: &mut VM, _op: StackOps, _bop: CrossbusOperations, _err_prefix: String) -> Result<&mut VM, Error> {
    Ok(vm)
}

pub fn stdlib_bund_bus_send_stack(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_bund_crossbus_base(vm, StackOps::FromStack, CrossbusOperations::Send, "SEND".to_string())
}

pub fn stdlib_bund_bus_send_workbench(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_bund_crossbus_base(vm, StackOps::FromWorkBench, CrossbusOperations::Send, "SEND.".to_string())
}

pub fn stdlib_bund_bus_recv_stack(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_bund_crossbus_base(vm, StackOps::FromStack, CrossbusOperations::Recv, "RECV".to_string())
}

pub fn stdlib_bund_bus_recv_workbench(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_bund_crossbus_base(vm, StackOps::FromWorkBench, CrossbusOperations::Recv, "RECV.".to_string())
}

pub fn stdlib_bund_bus_disabled(_vm: &mut VM) -> Result<&mut VM, Error> {
    bail!("bund BUS functions disabled with --noio");
}

pub fn init_stdlib(cli: &cmd::Cli) {
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };
    if cli.noio {
        let _ = bc.vm.register_inline("send".to_string(), stdlib_bund_bus_disabled);
        let _ = bc.vm.register_inline("send.".to_string(), stdlib_bund_bus_disabled);
        let _ = bc.vm.register_inline("recv".to_string(), stdlib_bund_bus_disabled);
        let _ = bc.vm.register_inline("recv.".to_string(), stdlib_bund_bus_disabled);
    } else {
        let _ = bc.vm.register_inline("send".to_string(), stdlib_bund_bus_send_stack);
        let _ = bc.vm.register_inline("send.".to_string(), stdlib_bund_bus_send_workbench);
        let _ = bc.vm.register_inline("recv".to_string(), stdlib_bund_bus_recv_stack);
        let _ = bc.vm.register_inline("recv.".to_string(), stdlib_bund_bus_recv_workbench);
    }
    drop(bc);
}
