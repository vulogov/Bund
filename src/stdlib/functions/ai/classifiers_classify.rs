extern crate log;
use rust_multistackvm::multistackvm::{VM};
use crate::stdlib::functions::ai::naivebayes;
use crate::stdlib::functions::ai::linguistic;
use easy_error::{Error, bail};
use crate::stdlib::functions::ai::{NN, NNType};


#[time_graph::instrument]
pub fn stdlib_classify_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 2 {
        bail!("Stack is too shallow for inline CLASSIFIERS.CLASSIFY");
    }
    match vm.stack.pull() {
        Some(data) => {
            let name_value = match vm.stack.pull() {
                Some(name) => name,
                None => bail!("CLASSIFIERS.CLASSIFY returns: NO DATA #2"),
            };
            let name = match name_value.cast_string() {
                Ok(name) => name,
                Err(err) => bail!("CLASSIFIERS.CLASSIFY casting a name returns: {}", err),
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
                match nn.id {
                    NNType::NaiveBayes => {
                        drop(ai);
                        return naivebayes::classify_naive_bayes_classifier(vm, name, data);
                    }
                    NNType::LangClassifier => {
                        drop(ai);
                        return linguistic::classify_linguistic_classifier(vm, name, data);
                    }
                    _ => {
                        drop(ai);
                        bail!("CLASSIFIERS.CLASSIFY classifier having an invalid type: {}", &name);
                    }
                }
            } else {
                drop(ai);
                bail!("CLASSIFIERS.CLASSIFY not found classifier: {}", &name);
            }
        }
        None => {
            bail!("CLASSIFIERS.CLASSIFY returns: NO DATA #1");
        }
    }
}
