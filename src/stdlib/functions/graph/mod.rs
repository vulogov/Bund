extern crate log;
use crate::stdlib::BUND;
use rust_multistackvm::multistackvm::{VM, StackOps};
use rust_dynamic::value::Value;
use rust_dynamic::types::*;
use crate::cmd;
use crate::stdlib::helpers;
use easy_error::{Error, bail};

pub mod make_graph;
pub mod dijkstra;
pub mod transitiveclosure;
pub mod allshortpath;
pub mod getpath;

pub fn stdlib_make_graph_base(vm: &mut VM, op: StackOps, err_prefix: String) -> Result<&mut VM, Error> {
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
    let mut graph_val = match data_val {
        Some(graph_val) => graph_val,
        None => bail!("{}: NO DATA #1", &err_prefix),
    };
    let type_val = match graph_val.get("type") {
        Ok(type_val) => type_val,
        Err(_) => Value::from_string("graph".to_string()),
    };
    let nodes_val = match graph_val.get("nodes") {
        Ok(nodes_val) => nodes_val,
        Err(_) => Value::list(),
    };
    let vertex_val = match graph_val.get("edges") {
        Ok(vertex_val) => vertex_val,
        Err(_) => Value::list(),
    };
    if nodes_val.dt != LIST {
        bail!("{}: Nodes list must be a list", &err_prefix);
    }
    if vertex_val.dt != LIST {
        bail!("{}: Edges list must be a list", &err_prefix);
    }
    graph_val = graph_val.set("type", type_val);
    graph_val = graph_val.set("nodes", nodes_val);
    graph_val = graph_val.set("edges", vertex_val);
    let _ = match op {
        StackOps::FromStack => vm.stack.push(graph_val),
        StackOps::FromWorkBench => vm.stack.push_to_workbench(graph_val),
    };

    Ok(vm)
}

#[time_graph::instrument]
pub fn stdlib_make_graph_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    let nodes_val = match vm.stack.pull() {
        Some(nodes_val) => if nodes_val.dt == LIST {
                nodes_val
            } else {
                vm.stack.push(nodes_val);
                Value::list()
        },
        None => Value::list(),
    };
    let edges_val = match vm.stack.pull() {
        Some(edges_val) => if edges_val.dt == LIST {
                edges_val
            } else {
                vm.stack.push(edges_val);
                Value::list()
        },
        None => Value::list(),
    };
    let mut res = Value::dict();
    res = res.set("type", Value::from_string("graph"));
    res = res.set("nodes", nodes_val);
    res = res.set("edges", edges_val);
    vm.stack.push(res);
    Ok(vm)
}

#[time_graph::instrument]
pub fn stdlib_make_graph_stack(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_make_graph_base(vm, StackOps::FromStack, "GRAPH".to_string())
}

#[time_graph::instrument]
pub fn stdlib_make_graph_wb(vm: &mut VM) -> Result<&mut VM, Error> {
    stdlib_make_graph_base(vm, StackOps::FromWorkBench, "GRAPH.".to_string())
}

pub fn init_stdlib(cli: &cmd::Cli) {
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };
    let _ = bc.vm.register_inline("graph".to_string(), stdlib_make_graph_stack);
    let _ = bc.vm.register_inline("graph.".to_string(), stdlib_make_graph_wb);
    let _ = bc.vm.register_inline("graph!".to_string(), stdlib_make_graph_inline);
    drop(bc);
    dijkstra::init_stdlib(cli);
    transitiveclosure::init_stdlib(cli);
    allshortpath::init_stdlib(cli);
    getpath::init_stdlib(cli);
}
