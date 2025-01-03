extern crate log;
use rust_multistackvm::multistackvm::{VM};
use crate::stdlib::helpers;
use rust_dynamic::value::Value;
use crate::stdlib::functions::ai::perceptron;
use easy_error::{Error, bail};

fn prepare_data_for_perceptron(vm: &mut VM, data: Value) -> Result<Vec<tensor::Tensor>, Error> {
    let input_data = match data.cast_list() {
        Ok(input_data) => input_data,
        Err(err) => bail!("Invalid input data #1 in NEURALNETWORKS.PREDICT: {}", err),
    };
    let mut data: Vec<tensor::Tensor> = Vec::new();
    let mut i_data: Vec<f32> = Nec::new();
    for v in input_data {
        let value = match v.cast_float() {
            Ok(value) => value,
            Err(err) => bail!("Invalid data in input NEURALNETWORKS.PREDICT: {}", err),
        };
        i_data.push(value as f32);
    }
    return Ok(data);
}

pub fn stdlib_neuralnetworks_predict_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline NEURALNETWORKS.PREDICT");
    }
    match vm.stack.pull() {
        Some(data) => {

        }
        None => {
            bail!("NEURALNETWORKS.PREDICT returns: NO DATA #1");
        }
    }
}
