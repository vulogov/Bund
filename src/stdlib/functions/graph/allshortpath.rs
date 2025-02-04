extern crate log;
use crate::stdlib::BUND;
use rust_multistackvm::multistackvm::{VM};
use crate::cmd;
use rust_dynamic::value::Value;
use rust_dynamic::types::*;
use crate::stdlib::functions::graph;
use algos::cs::graph::johnson;
use crate::stdlib::helpers;
use easy_error::{Error, bail};

#[time_graph::instrument]
pub fn stdlib_graph_allpath_stack(vm: &mut VM) -> Result<&mut VM, Error> {
    let g = match vm.stack.pull() {
        Some(g_val) => g_val,
        None => bail!("GRAPH.ALLPATH: NO DATA #1"),
    };
    let type_val = match g.get("type") {
        Ok(type_val) => type_val,
        Err(_) => bail!("GRAPH.ALLPATH: MISSED TYPE"),
    };
    let type_str = match type_val.cast_string() {
        Ok(type_str) => type_str,
        Err(err) => bail!("GRAPH.ALLPATH: type casting returns: {}", err),
    };
    match type_str.as_str() {
        "graph" => {},
        _ => bail!("GRAPH.ALLPATH: unknown data type"),
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
        bail!("GRAPH.ALLPATH: Nodes list must be a list");
    }
    if vertex_val.dt != LIST {
        bail!("GRAPH.ALLPATH: Edges list must be a list");
    }
    let (_t, t_res, graph) = match graph::make_graph::make_graph(&nodes_val, &vertex_val) {
        Ok((t, t_res, graph)) => (t, t_res, graph),
        Err(err) => bail!("{}", err),
    };

    match johnson::all_pairs_shortest_paths(&graph) {
        Ok(d_res) => {
            let mut res = Value::list();
            for (k, v) in d_res {
                let mut row = Value::dict();
                match v {
                    Some(w) => {
                        row = row.set("weight", Value::from_float(w));
                    }
                    None => {
                        continue;
                    }
                }
                let (from_ix, to_ix) = k;
                let from_name = match t_res.get(&from_ix) {
                    Some(n_name) => n_name,
                    None => bail!("GRAPH.TRANSITIVECLOSURE: unknown node {}", from_ix),
                };
                let to_name = match t_res.get(&to_ix) {
                    Some(n_name) => n_name,
                    None => bail!("GRAPH.TRANSITIVECLOSURE: unknown node {}", to_ix),
                };
                row = row.set("from", Value::from_string(from_name));
                row = row.set("to", Value::from_string(to_name));
                if from_ix == to_ix {
                    row = row.set("itself", Value::from_bool(true));
                } else {
                    row = row.set("itself", Value::from_bool(false));
                }
                res = res.push(row);
            }
            vm.stack.push(res);
        }
        Err(err) => bail!("GRAPH.ALLPATH: returned: {}", err),
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
    let _ = bc.vm.register_inline("graph.allpath".to_string(), stdlib_graph_allpath_stack);
}
