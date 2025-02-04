extern crate log;
use rust_dynamic::value::Value;
use algos::cs::graph::Graph;
use fast_paths::InputGraph;
use convert_case::{Case, Casing};
use std::collections::HashMap;
use easy_error::{Error, bail};

fn make_nodename(name: String) -> String {
    name.clone().trim().to_case(Case::Title)
}

pub fn make_fast_graph(nodes: &Value, edges: &Value) -> Result<(HashMap<String, usize>, HashMap<usize, String>, InputGraph), Error> {
    let mut t: HashMap<String, usize> = HashMap::new();
    let mut t_res: HashMap<usize, String> = HashMap::new();
    let mut g = InputGraph::new();
    let mut c: usize = 0;
    let nodes_list = match nodes.cast_list() {
        Ok(nodes_list) => nodes_list,
        Err(err) => bail!("MAKE_FAST_GRAPH: error casting nodes list: {}", err),
    };
    for n in nodes_list {
        let node_name = match n.cast_string() {
            Ok(node_name) => make_nodename(node_name),
            Err(err) => bail!("MAKE_FAST_GRAPH: error casting node name: {}", err),
        };
        if ! t.contains_key(&node_name) {
            t.insert(node_name.clone(), c);
            t_res.insert(c, node_name.clone());
            c += 1;
        }
    }
    let edges_list = match edges.cast_list() {
        Ok(edges_list) => edges_list,
        Err(err) => bail!("MAKE_FAST_GRAPH: error casting edges list: {}", err),
    };
    for e in edges_list {
        match e.cast_list() {
            Ok(rel) => if rel.len() >= 2 {
                let from_node = match rel[0].cast_string() {
                    Ok(from_node) => make_nodename(from_node),
                    Err(err) => bail!("MAKE_FAST_GRAPH: error casting from node name: {}", err),
                };
                let to_node = match rel[1].cast_string() {
                    Ok(from_node) => make_nodename(from_node),
                    Err(err) => bail!("NAME_GRAPH: error casting from node name: {}", err),
                };
                if ! t.contains_key(&from_node) || ! t.contains_key(&to_node) {
                    bail!("MAKE_FAST_GRAPH: {} and {} unknown nodes", &from_node, &to_node);
                }
                let from_ix = match t.get(&from_node) {
                    Some(from_ix) => from_ix,
                    None => bail!("MAKE_FAST_GRAPH: unknown node: {}", &from_node),
                };
                let to_ix = match t.get(&to_node) {
                    Some(to_ix) => to_ix,
                    None => bail!("MAKE_FAST_GRAPH: unknown node: {}", &from_node),
                };
                if rel.len() == 3 {
                    let w = match rel[2].cast_float() {
                        Ok(w) => w,
                        Err(err) => bail!("MAKE_FAST_GRAPH: Error casting weight: {}", err),
                    };
                    g.add_edge(*from_ix, *to_ix, w as usize);
                } else {
                    g.add_edge(*from_ix, *to_ix, 100 as usize);
                }
            } else {
                bail!("MAKE_FAST_GRAPH: relation is too small");
            },
            Err(err) => bail!("MAKE_FAST_GRAPH: error casting relation: {}", err),
        };
    }
    g.freeze();
    Ok((t, t_res, g))
}

pub fn make_graph(nodes: &Value, edges: &Value) -> Result<(HashMap<String, usize>, HashMap<usize, String>, Graph<usize, f64>), Error> {
    let mut t: HashMap<String, usize> = HashMap::new();
    let mut t_res: HashMap<usize, String> = HashMap::new();
    let mut g: Graph<usize, f64> = Graph::new();
    let mut c: usize = 0;
    let nodes_list = match nodes.cast_list() {
        Ok(nodes_list) => nodes_list,
        Err(err) => bail!("MAKE_GRAPH: error casting nodes list: {}", err),
    };
    for n in nodes_list {
        let node_name = match n.cast_string() {
            Ok(node_name) => make_nodename(node_name),
            Err(err) => bail!("MAKE_GRAPH: error casting node name: {}", err),
        };
        if ! t.contains_key(&node_name) {
            t.insert(node_name.clone(), c);
            t_res.insert(c, node_name.clone());
            g.add_vertex(c);
            c += 1;
        }
    }
    let edges_list = match edges.cast_list() {
        Ok(edges_list) => edges_list,
        Err(err) => bail!("MAKE_GRAPH: error casting edges list: {}", err),
    };
    for e in edges_list {
        match e.cast_list() {
            Ok(rel) => if rel.len() >= 2 {
                let from_node = match rel[0].cast_string() {
                    Ok(from_node) => make_nodename(from_node),
                    Err(err) => bail!("NAME_GRAPH: error casting from node name: {}", err),
                };
                let to_node = match rel[1].cast_string() {
                    Ok(from_node) => make_nodename(from_node),
                    Err(err) => bail!("MAKE_GRAPH: error casting from node name: {}", err),
                };
                if ! t.contains_key(&from_node) || ! t.contains_key(&to_node) {
                    bail!("MAKE_GRAPH: {} and {} unknown nodes", &from_node, &to_node);
                }
                let from_ix = match t.get(&from_node) {
                    Some(from_ix) => from_ix,
                    None => bail!("MAKE_GRAPH: unknown node: {}", &from_node),
                };
                let to_ix = match t.get(&to_node) {
                    Some(to_ix) => to_ix,
                    None => bail!("MAKE_GRAPH: unknown node: {}", &from_node),
                };
                if rel.len() == 3 {
                    let w = match rel[2].cast_float() {
                        Ok(w) => w,
                        Err(err) => bail!("MAKE_GRAPH: Error casting weight: {}", err),
                    };
                    g.add_edge(*from_ix, *to_ix, w);
                } else {
                    g.add_edge(*from_ix, *to_ix, 100.0);
                }
            } else {
                bail!("MAKE_GRAPH: relation is too small");
            },
            Err(err) => bail!("MAKE_GRAPH: error casting relation: {}", err),
        };
    }
    Ok((t, t_res, g))
}
