extern crate log;
use crate::cmd;
use crate::stdlib::helpers;
use std::sync::Mutex;
use std::collections::btree_map::BTreeMap;
use lazy_static::lazy_static;
use neurons::{network};
use natural::classifier::NaiveBayesClassifier;
use crate::stdlib::BUND;

pub mod perceptron;
pub mod neuralnetworks;
pub mod neuralnetworks_predict;
pub mod classifiers;
pub mod classifiers_classify;
pub mod naivebayes;
pub mod linguistic;

#[derive(Clone, Debug)]
pub enum NNType {
    Perceptron,
    NaiveBayes,
    LangClassifier,
}

pub enum NNVal {
    Null,
    Perceptron(network::Network),
    NaiveBayes(NaiveBayesClassifier)
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
    let _ = bc.vm.register_inline("classifier".to_string(), classifiers::stdlib_classifier_inline);
    let _ = bc.vm.register_inline("neuralnetwork.predict".to_string(), neuralnetworks_predict::stdlib_neuralnetworks_predict_inline);
    let _ = bc.vm.register_inline("classifier.classify".to_string(), classifiers_classify::stdlib_classify_inline);
    drop(bc);
}
