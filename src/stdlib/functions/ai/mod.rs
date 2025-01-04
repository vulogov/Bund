extern crate log;
use crate::cmd;
use crate::stdlib::helpers;
use std::sync::Mutex;
use std::collections::btree_map::BTreeMap;
use lazy_static::lazy_static;
use neurons::{network};
use crate::stdlib::BUND;

pub mod perceptron;
pub mod neuralnetworks;
pub mod neuralnetworks_predict;

#[derive(Clone, Debug)]
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

pub fn init_stdlib(cli: &cmd::Cli) {
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };

    let _ = bc.vm.register_inline("neuralnetwork".to_string(), neuralnetworks::stdlib_neuralnetworks_inline);
    let _ = bc.vm.register_inline("neuralnetwork.predict".to_string(), neuralnetworks_predict::stdlib_neuralnetworks_predict_inline);
    drop(bc);
}
