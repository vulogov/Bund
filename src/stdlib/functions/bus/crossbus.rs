extern crate log;
use crate::stdlib::BUND;
use rust_dynamic::value::Value;
use rust_multistackvm::multistackvm::{VM, StackOps};
use crate::stdlib::functions::bus;
use crate::cmd;
use crate::stdlib::helpers;
use easy_error::{Error, bail};

#[derive(Debug, Clone)]
pub enum CrossbusOperations {
    Send,
    Recv,
}

fn stdlib_bund_crossbus_bus_have_data_current(vm: &mut VM) -> Result<&mut VM, Error> {
    let name = match vm.stack.current_stack_name() {
        Some(name) => name,
        None => bail!("Can not determine current stack name"),
    };
    vm.stack.push(Value::from_bool(bus::ensure_bus(name)));
    Ok(vm)
}

fn stdlib_bund_crossbus_bus_have_data(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for BUS.DATA");
    }
    let name_val = match vm.stack.pull() {
        Some(name_val) => name_val,
        None => bail!("BUS.DATA: No channel name discovered on the stack"),
    };
    let name = match name_val.cast_string() {
        Ok(name) => name,
        Err(err) => bail!("BUS.DATA: Error name casting: {}", err),
    };
    vm.stack.push(Value::from_bool(bus::ensure_bus(name)));
    Ok(vm)
}

fn stdlib_bund_crossbus_bus_send_data(vm: &mut VM, op: StackOps, err_prefix: String) -> Result<&mut VM, Error> {
    match op {
        StackOps::FromStack => {
            if vm.stack.current_stack_len() < 2 {
                bail!("Stack is too shallow for inline {}", &err_prefix);
            }
        }
        StackOps::FromWorkBench => {
            if vm.stack.current_stack_len() < 1 {
                bail!("Stack is too shallow for inline {}", &err_prefix);
            }
            if vm.stack.workbench.len() < 1 {
                bail!("Workbench is too shallow for inline {}", &err_prefix);
            }
        }
    }
    let object_val = match op {
        StackOps::FromStack => vm.stack.pull(),
        StackOps::FromWorkBench => vm.stack.pull_from_workbench(),
    };
    let object = match object_val {
        Some(object) => object,
        None => bail!("{}: No object for sending was provided", &err_prefix),
    };
    let name_val = match vm.stack.pull() {
        Some(name_val) => name_val,
        None => bail!("{}: No channel name discovered on the stack", &err_prefix),
    };
    let name = match name_val.cast_string() {
        Ok(name) => name,
        Err(err) => bail!("BUS.DATA: Error name casting: {}", err),
    };
    match bus::bus_push(name, object) {
        Ok(res) => vm.stack.push(Value::from_bool(res)),
        Err(err) => bail!("{} returns error {}", &err_prefix, err),
    };
    Ok(vm)
}

fn stdlib_bund_crossbus_bus_quick_send_data(vm: &mut VM, op: StackOps, err_prefix: String) -> Result<&mut VM, Error> {
    match op {
        StackOps::FromStack => {
            if vm.stack.current_stack_len() < 2 {
                bail!("Stack is too shallow for inline {}", &err_prefix);
            }
        }
        StackOps::FromWorkBench => {
            if vm.stack.current_stack_len() < 1 {
                bail!("Stack is too shallow for inline {}", &err_prefix);
            }
            if vm.stack.workbench.len() < 1 {
                bail!("Workbench is too shallow for inline {}", &err_prefix);
            }
        }
    }
    let object_val = match op {
        StackOps::FromStack => vm.stack.pull(),
        StackOps::FromWorkBench => vm.stack.pull_from_workbench(),
    };
    let object = match object_val {
        Some(object) => object,
        None => bail!("{}: No object for sending was provided", &err_prefix),
    };
    let name_val = match vm.stack.pull() {
        Some(name_val) => name_val,
        None => bail!("{}: No channel name discovered on the stack", &err_prefix),
    };
    let name = match name_val.cast_string() {
        Ok(name) => name,
        Err(err) => bail!("BUS.DATA: Error name casting: {}", err),
    };
    match bus::bus_push(name, object) {
        Ok(_) => {},
        Err(err) => bail!("{} returns error {}", &err_prefix, err),
    };
    Ok(vm)
}

fn stdlib_bund_crossbus_bus_recv_data(vm: &mut VM, op: StackOps, err_prefix: String) -> Result<&mut VM, Error> {
    match op {
        StackOps::FromStack => {
            if vm.stack.current_stack_len() < 1 {
                bail!("Stack is too shallow for inline {}", &err_prefix);
            }
        }
        StackOps::FromWorkBench => {
            if vm.stack.workbench.len() < 1 {
                bail!("Workbench is too shallow for inline {}", &err_prefix);
            }
        }
    }
    let name_obj = match op {
        StackOps::FromStack => vm.stack.pull(),
        StackOps::FromWorkBench => vm.stack.pull_from_workbench(),
    };
    let name_val = match name_obj {
        Some(name_val) => name_val,
        None => bail!("{}: No channel name discovered on the stack", &err_prefix),
    };
    let name = match name_val.cast_string() {
        Ok(name) => name,
        Err(err) => bail!("BUS.DATA: Error name casting: {}", err),
    };
    match bus::bus_pull(name) {
        Ok(res) => vm.stack.push(res),
        Err(err) => bail!("{} returns error {}", &err_prefix, err),
    };
    Ok(vm)
}

fn stdlib_bund_crossbus_base(vm: &mut VM, op: StackOps, bop: CrossbusOperations, err_prefix: String) -> Result<&mut VM, Error> {
    match bop {
        CrossbusOperations::Send => stdlib_bund_crossbus_bus_send_data(vm, op, err_prefix),
        CrossbusOperations::Recv => stdlib_bund_crossbus_bus_recv_data(vm, op, err_prefix),
    }
}

fn stdlib_bund_crossbus_quick_base(vm: &mut VM, op: StackOps, bop: CrossbusOperations, err_prefix: String) -> Result<&mut VM, Error> {
    match bop {
        CrossbusOperations::Send => stdlib_bund_crossbus_bus_quick_send_data(vm, op, err_prefix),
        CrossbusOperations::Recv => stdlib_bund_crossbus_bus_recv_data(vm, op, err_prefix),
    }
}

#[time_graph::instrument]
pub fn stdlib_bund_bus_send_stack(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_bund_crossbus_base(vm, StackOps::FromStack, CrossbusOperations::Send, "SEND".to_string())
}

#[time_graph::instrument]
pub fn stdlib_bund_bus_send_workbench(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_bund_crossbus_base(vm, StackOps::FromWorkBench, CrossbusOperations::Send, "SEND.".to_string())
}

#[time_graph::instrument]
pub fn stdlib_bund_bus_send_stack_quick(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_bund_crossbus_quick_base(vm, StackOps::FromStack, CrossbusOperations::Send, "SEND".to_string())
}

#[time_graph::instrument]
pub fn stdlib_bund_bus_send_workbench_quick(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_bund_crossbus_quick_base(vm, StackOps::FromWorkBench, CrossbusOperations::Send, "SEND.".to_string())
}

#[time_graph::instrument]
pub fn stdlib_bund_bus_recv_stack(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_bund_crossbus_base(vm, StackOps::FromStack, CrossbusOperations::Recv, "RECV".to_string())
}

#[time_graph::instrument]
pub fn stdlib_bund_bus_recv_workbench(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_bund_crossbus_base(vm, StackOps::FromWorkBench, CrossbusOperations::Recv, "RECV.".to_string())
}

#[time_graph::instrument]
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
        let _ = bc.vm.register_inline("send.quick".to_string(), stdlib_bund_bus_disabled);
        let _ = bc.vm.register_inline("send.".to_string(), stdlib_bund_bus_disabled);
        let _ = bc.vm.register_inline("send.quick.".to_string(), stdlib_bund_bus_disabled);
        let _ = bc.vm.register_inline("recv".to_string(), stdlib_bund_bus_disabled);
        let _ = bc.vm.register_inline("recv.".to_string(), stdlib_bund_bus_disabled);
        let _ = bc.vm.register_inline("bus.data".to_string(), stdlib_bund_bus_disabled);
        let _ = bc.vm.register_inline("bus.data.current".to_string(), stdlib_bund_bus_disabled);
    } else {
        let _ = bc.vm.register_inline("send".to_string(), stdlib_bund_bus_send_stack);
        let _ = bc.vm.register_inline("send.quick".to_string(), stdlib_bund_bus_send_stack_quick);
        let _ = bc.vm.register_inline("send.".to_string(), stdlib_bund_bus_send_workbench);
        let _ = bc.vm.register_inline("send.quick.".to_string(), stdlib_bund_bus_send_workbench_quick);
        let _ = bc.vm.register_inline("recv".to_string(), stdlib_bund_bus_recv_stack);
        let _ = bc.vm.register_inline("recv.".to_string(), stdlib_bund_bus_recv_workbench);
        let _ = bc.vm.register_inline("bus.data.current".to_string(), stdlib_bund_crossbus_bus_have_data_current);
        let _ = bc.vm.register_inline("bus.data".to_string(), stdlib_bund_crossbus_bus_have_data);
    }
    drop(bc);
}
