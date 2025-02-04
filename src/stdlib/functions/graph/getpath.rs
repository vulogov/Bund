extern crate log;
use crate::stdlib::BUND;
use rust_multistackvm::multistackvm::{VM};
use crate::cmd;
use rust_dynamic::value::Value;
use rust_dynamic::types::*;
use crate::stdlib::functions::graph;
use fast_paths;
use crate::stdlib::helpers;
use easy_error::{Error, bail};

#[time_graph::instrument]
pub fn stdlib_graph_fastpath_stack(vm: &mut VM) -> Result<&mut VM, Error> {
    let g = match vm.stack.pull() {
        Some(g_val) => g_val,
        None => bail!("GRAPH.PATH: NO DATA #1"),
    };
    let type_val = match g.get("type") {
        Ok(type_val) => type_val,
        Err(_) => bail!("GRAPH.PATH: MISSED TYPE"),
    };
    let type_str = match type_val.cast_string() {
        Ok(type_str) => type_str,
        Err(err) => bail!("GRAPH.PATH: type casting returns: {}", err),
    };
    match type_str.as_str() {
        "graph" => {},
        _ => bail!("GRAPH.PATH: unknown data type"),
    };
    let nodes_val = match g.get("nodes") {
        Ok(nodes_val) => nodes_val,
        Err(_) => Value::list(),
    };
    let vertex_val = match g.get("edges") {
        Ok(vertex_val) => vertex_val,
        Err(_) => Value::list(),
    };
    if nodes_val.dt != LIST {
        bail!("GRAPH.PATH: Nodes list must be a list");
    }
    if vertex_val.dt != LIST {
        bail!("GRAPH.PATH: Edges list must be a list");
    }
    let (t, t_res, graph) = match graph::make_graph::make_fast_graph(&nodes_val, &vertex_val) {
        Ok((t, t_res, graph)) => (t, t_res, graph),
        Err(err) => bail!("{}", err),
    };
    let fgraph = fast_paths::prepare(&graph);
    let start_node_val = match vm.stack.pull() {
        Some(n_val) => n_val,
        None => bail!("GRAPH.PATH: NO DATA #2"),
    };
    let start_node = match start_node_val.cast_string() {
        Ok(start_node) => start_node,
        Err(err) => bail!("GRAPH.PATH: casting start node name returns: {}", err),
    };
    let start_node_ix = match t.get(&start_node) {
        Some(start_node_ix) => start_node_ix,
        None => bail!("GRAPH.PATH: unknown start node: {}", &start_node),
    };
    let end_node_val = match vm.stack.pull() {
        Some(n_val) => n_val,
        None => bail!("GRAPH.PATH: NO DATA #3"),
    };
    let end_node = match end_node_val.cast_string() {
        Ok(start_node) => start_node,
        Err(err) => bail!("GRAPH.PATH: casting start node name returns: {}", err),
    };
    let end_node_ix = match t.get(&end_node) {
        Some(start_node_ix) => start_node_ix,
        None => bail!("GRAPH.PATH: unknown end node: {}", &start_node),
    };
    match fast_paths::calc_path(&fgraph, *start_node_ix, *end_node_ix) {
        Some(d_res) => {
            let mut res = Value::dict();
            if ! d_res.is_found() {
                res = res.set("found", Value::make_false());
            } else {
                let mut p = Value::list();
                for n in d_res.get_nodes() {
                    let node_name = match t_res.get(&n) {
                        Some(n_name) => n_name,
                        None => bail!("GRAPH.PATH: unknown node {}", &n),
                    };
                    p = p.push(Value::from_string(node_name));
                }
                res = res.set("path", p);
                res = res.set("weight", Value::from_float(d_res.get_weight() as f64));
                res = res.set("found", Value::make_true());
            }
            vm.stack.push(res);
        }
        None => bail!("GRAPH.PATH: can not calculate path"),
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
    let _ = bc.vm.register_inline("graph.path".to_string(), stdlib_graph_fastpath_stack);
}
