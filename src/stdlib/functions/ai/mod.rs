extern crate log;
use crate::cmd;
use std::sync::Mutex;
use std::collections::btree_map::BTreeMap;
use lazy_static::lazy_static;
use neurons::{network};

pub mod perceptron;

#[derive(Clone)]
pub enum NNType {
    Perceptron,
}

pub enum NNVal {
    Null,
    Perceptron(network::Network),
}


pub struct NNEntry {
    pub id:     NNType,
    pub nn:     NNVal,
}

lazy_static! {
    static ref NN: Mutex<BTreeMap<String, NNEntry>> = {
        let e: Mutex<BTreeMap<String, NNEntry>> = Mutex::new(BTreeMap::new());
        e
    };
}

pub fn init_stdlib(_cli: &cmd::Cli) {

}
