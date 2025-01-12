extern crate log;
use crate::stdlib::BUND;
use rust_dynamic::value::Value;
use rust_multistackvm::multistackvm::{VM, StackOps};
use crate::cmd;
use crate::stdlib::helpers;
use easy_error::{Error, bail};
use rasciigraph;

pub fn bund_io_graph_base(vm: &mut VM, op: StackOps, err_prefix: String) -> Result<&mut VM, Error> {
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

    let data_val = match op {
        StackOps::FromStack => vm.stack.pull(),
        StackOps::FromWorkBench => vm.stack.pull_from_workbench(),
    };

    match data_val {
        Some(data_val) => {
            match data_val.cast_list() {
                Ok(data_list) => {
                    let mut res:Vec<f64> = Vec::new();
                    for i in data_list {
                        match i.cast_float() {
                            Ok(data) => {
                                res.push(data);
                            }
                            Err(err) => {
                                bail!("{} casting data element returns: {}", &err_prefix, err);
                            }
                        }
                    }
                    let out = rasciigraph::plot(
                        res,
                        rasciigraph::Config::default()
                    );
                    vm.stack.push(Value::from_string(out));
                }
                Err(err) => {
                    bail!("{} casting data returns: {}", &err_prefix, err);
                }
            }
        }
        None => {
            bail!("{} returns: NO DATA #1", &err_prefix);
        }
    }
    Ok(vm)
}

pub fn stdlib_bund_io_graph_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    bund_io_graph_base(vm, StackOps::FromStack, "IO.GRAPH".to_string())
}

pub fn stdlib_bund_io_graph_from_wrokbench_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    bund_io_graph_base(vm, StackOps::FromWorkBench, "IO.GRAPH.".to_string())
}

pub fn init_stdlib(cli: &cmd::Cli) {
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };
    let _ = bc.vm.register_inline("io.graph".to_string(), stdlib_bund_io_graph_inline);
    let _ = bc.vm.register_inline("io.graph.".to_string(), stdlib_bund_io_graph_from_wrokbench_inline);

    drop(bc);
}
