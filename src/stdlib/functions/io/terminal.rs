extern crate log;
use crate::stdlib::BUND;
use rust_multistackvm::multistackvm::{VM, StackOps};
use crate::cmd;
use crate::stdlib::helpers;
use easy_error::{Error};


#[derive(Debug, Clone)]
pub enum TerminalOp {
    Line,
}

pub fn terminal_op_base(vm: &mut VM, _op: StackOps, top: TerminalOp, _err_prefix: String) -> Result<&mut VM, Error> {
    match top {
        TerminalOp::Line => {},
    };
    Ok(vm)
}

#[time_graph::instrument]
pub fn stdlib_terminal_line_stack(vm: &mut VM) -> Result<&mut VM, Error> {
    terminal_op_base(vm, StackOps::FromStack, TerminalOp::Line, "TERMINAL.LINE".to_string())
}

#[time_graph::instrument]
pub fn stdlib_terminal_line_workbench(vm: &mut VM) -> Result<&mut VM, Error> {
    terminal_op_base(vm, StackOps::FromWorkBench, TerminalOp::Line, "TERMINAL.LINE.".to_string())
}

pub fn init_stdlib(cli: &cmd::Cli) {
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };
    let _ = bc.vm.register_inline("terminal.line".to_string(), stdlib_terminal_line_stack);
    let _ = bc.vm.register_inline("terminal.line.".to_string(), stdlib_terminal_line_workbench);

    drop(bc);
}
