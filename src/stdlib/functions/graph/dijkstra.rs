extern crate log;
use crate::stdlib::BUND;
use rust_multistackvm::multistackvm::{VM};
use crate::cmd;
use rust_dynamic::value::Value;
use rust_dynamic::types::*;
use crate::stdlib::functions::graph;
use algos::cs::graph::dijkstra;
use crate::stdlib::helpers;
use easy_error::{Error, bail};

#[time_graph::instrument]
pub fn stdlib_graph_dijkstra_stack(vm: &mut VM) -> Result<&mut VM, Error> {
    let g = match vm.stack.pull() {
        Some(g_val) => g_val,
        None => bail!("GRAPH.PATHS: NO DATA #1"),
    };
    let type_val = match g.get("type") {
        Ok(type_val) => type_val,
        Err(_) => bail!("GRAPH.PATHS: MISSED TYPE"),
    };
    let type_str = match type_val.cast_string() {
        Ok(type_str) => type_str,
        Err(err) => bail!("GRAPH.PATHS: type casting returns: {}", err),
    };
    match type_str.as_str() {
        "graph" => {},
        _ => bail!("GRAPH.PATHS: unknown data type"),
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
        bail!("GRAPH.PATHS: Nodes list must be a list");
    }
    if vertex_val.dt != LIST {
        bail!("GRAPH.PATHS: Edges list must be a list");
    }
    let (t, t_res, graph) = match graph::make_graph::make_graph(&nodes_val, &vertex_val) {
        Ok((t, t_res, graph)) => (t, t_res, graph),
        Err(err) => bail!("{}", err),
    };
    let start_node_val = match vm.stack.pull() {
        Some(n_val) => n_val,
        None => bail!("GRAPH.PATHS: NO DATA #2"),
    };
    let start_node = match start_node_val.cast_string() {
        Ok(start_node) => start_node,
        Err(err) => bail!("GRAPH.PATHS: casting start node name returns: {}", err),
    };
    let start_node_ix = match t.get(&start_node) {
        Some(start_node_ix) => start_node_ix,
        None => bail!("GRAPH.PATHS: unknown start node: {}", &start_node),
    };
    match dijkstra::shortest_paths(&graph, start_node_ix) {
        Ok(d_res) => {
            let mut res = Value::list();
            for (k, v) in d_res {
                let mut row = Value::dict();
                let n_name = match t_res.get(&k) {
                    Some(n_name) => n_name,
                    None => bail!("GRAPH.PATHS: unknown node {}", k),
                };
                row = row.set("node", Value::from_string(n_name));
                match v {
                    Some(w) => {
                        row = row.set("path", Value::from_float(w));
                    }
                    None => {
                        row = row.set("path", Value::nodata());
                    }
                }
                res = res.push(row);
            }
            vm.stack.push(res);
        }
        Err(err) => bail!("GRAPH.PATHS: returned: {}", err),
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
    let _ = bc.vm.register_inline("graph.paths".to_string(), stdlib_graph_dijkstra_stack);
}
