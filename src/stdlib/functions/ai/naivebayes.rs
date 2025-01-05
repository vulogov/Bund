extern crate log;
use std::collections::HashMap;
use crate::stdlib::functions::ai::{NN, NNEntry, NNType, NNVal};
use rust_multistackvm::multistackvm::{VM};
use crate::stdlib::helpers;
use rust_dynamic::value::Value;
use natural::classifier::NaiveBayesClassifier;
use easy_error::{Error, bail};

impl NNEntry {
    pub fn new_naivebayes(naive_bayes: NaiveBayesClassifier) -> Self {
        Self {
            id:     NNType::NaiveBayes,
            nn:     NNVal::NaiveBayes(naive_bayes),
        }
    }
}



fn prepare_training_data(vm: &mut VM, conf: Value, name: String) -> Result<HashMap<std::string::String, rust_dynamic::value::Value>, Error> {
    let data = helpers::conf::conf_get(vm, conf.clone(), name.to_string(), Value::dict()).cast_dict().unwrap();
    return Ok(data);
}

pub fn create_naivebayes_classifier(vm: &mut VM, name: String, conf: Value) -> Result<&mut VM, Error> {
    log::debug!("Create NaiveBayes classifier named: {}", &name);

    let mut nbc = NaiveBayesClassifier::new();

    let data = match prepare_training_data(vm, conf.clone(), "Data".to_string()) {
        Ok(x) => x,
        Err(err) => bail!("{}", err),
    };

    for (k,v) in data.iter() {
        match v.cast_list() {
            Ok(values) => {
                for x in values {
                    match x.cast_string() {
                        Ok(training_value) => {
                            nbc.train(&training_value, &k);
                        }
                        Err(err) => {
                            bail!("Casting training data for {} failed: {}", &k, err);
                        }
                    }
                }
            }
            Err(err) => {
                bail!("Casting list of training data for {} failed: {}", &k, err);
            }
        }
    }
    let mut ai = NN.lock().unwrap();
    ai.insert(name.to_string(), NNEntry::new_naivebayes(nbc));
    drop(ai);
    Ok(vm)
}

pub fn classify_naive_bayes_classifier(vm: &mut VM, name: String, input: Value) -> Result<&mut VM, Error> {
    let input_text = match input.cast_string() {
        Ok(input_text) => input_text,
        Err(err) => bail!("NAIVEBAYES classify returns error on input: {}", err),
    };
    let ai = NN.lock().unwrap();
    if ai.contains_key(&name) {
        let nn = match ai.get(&name) {
            Some(nn) => nn,
            None => {
                drop(ai);
                bail!("CLASSIFIERS.CLASSIFY not found classifier: {}. It is not there.", &name);
            },
        };
        match &nn.nn {
            NNVal::NaiveBayes(nbc) => {
                let guessing = nbc.guess(&input_text);
                vm.stack.push(Value::from_string(guessing));
            }
            _ => {
                drop(ai);
                bail!("CLASSIFIERS.CLASSIFY not resolve classifier: {}", &name);
            }
        }
    } else {
        drop(ai);
        bail!("CLASSIFIERS.CLASSIFY not found classifier: {}", &name);
    }
    drop(ai);
    Ok(vm)
}
