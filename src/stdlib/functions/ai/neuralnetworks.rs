extern crate log;
use rust_multistackvm::multistackvm::{VM};
use crate::stdlib::helpers;
use rust_dynamic::value::Value;
use crate::stdlib::functions::ai::perceptron;
use easy_error::{Error, bail};

#[time_graph::instrument]
pub fn stdlib_neuralnetworks_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 2 {
        bail!("Stack is too shallow for inline NEURALNETWORKS");
    }
    match vm.stack.pull() {
        Some(conf) => {
            let name_value = match vm.stack.pull() {
                Some(name_value) => name_value,
                None => bail!("NEURALNETWORKS returns: NO DATA #2")
            };
            let name: String = match name_value.cast_string() {
                Ok(name) => name,
                Err(err) => bail!("NEURALNETWORKS name casting returns: {}", err),
            };
            let nn_type = helpers::conf::conf_get(vm, conf.clone(), "type".to_string(), Value::from_string("unknown"));
            let res = match nn_type.cast_string().unwrap().as_str() {
                "perceptron" => perceptron::create_perceptron_nn(vm, name, conf),
                _ => bail!("Unknown NEURALNETWORK type: {}", &nn_type),
            };
            return res;
        }
        None => {
            bail!("NEURALNETWORKS returns: NO DATA #1");
        }
    }
}
