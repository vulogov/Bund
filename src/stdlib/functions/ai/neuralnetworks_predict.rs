extern crate log;
use rust_multistackvm::multistackvm::{VM};
use crate::stdlib::functions::ai::perceptron;
use easy_error::{Error, bail};
use crate::stdlib::functions::ai::{NN, NNType};


#[time_graph::instrument]
pub fn stdlib_neuralnetworks_predict_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 2 {
        bail!("Stack is too shallow for inline NEURALNETWORKS.PREDICT");
    }
    match vm.stack.pull() {
        Some(data) => {
            let name_value = match vm.stack.pull() {
                Some(name) => name,
                None => bail!("NEURALNETWORKS.PREDICT returns: NO DATA #2"),
            };
            let name = match name_value.cast_string() {
                Ok(name) => name,
                Err(err) => bail!("NEURALNETWORK.PREDICT casting a name returns: {}", err),
            };
            let ai = NN.lock().unwrap();
            if ai.contains_key(&name) {
                let nn = match ai.get(&name) {
                    Some(nn) => nn,
                    None => {
                        drop(ai);
                        bail!("NEURALNETWORKS.PREDICT not found network: {}. It is not there.", &name);
                    },
                };
                match nn.id {
                    NNType::Perceptron => {
                        drop(ai);
                        return perceptron::predict_perceptron_nn(vm, name, data);
                    }
                    _ => {
                        drop(ai);
                        bail!("NEURALNETWORKS.PREDICT network having an invalid type: {}", &name);
                    }
                }
            } else {
                drop(ai);
                bail!("NEURALNETWORKS.PREDICT not found network: {}", &name);
            }
        }
        None => {
            bail!("NEURALNETWORKS.PREDICT returns: NO DATA #1");
        }
    }
}
