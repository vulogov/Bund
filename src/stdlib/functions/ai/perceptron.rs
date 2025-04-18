extern crate log;
use rust_dynamic::types::*;
use crate::stdlib::functions::ai::{NN, NNEntry, NNType, NNVal};
use rust_multistackvm::multistackvm::{VM};
use crate::stdlib::helpers;
use rust_dynamic::value::Value;
use neurons::{activation, network, objective, optimizer, tensor};
use easy_error::{Error, bail};

impl NNEntry {
    pub fn new_perceptron(perceptron_network: network::Network) -> Self {
        Self {
            id:     NNType::Perceptron,
            nn:     NNVal::Perceptron(perceptron_network),
        }
    }
}

fn prepare_input_data_for_perceptron(_vm: &mut VM, data: Value) -> Result<tensor::Tensor, Error> {
    let input_data = match data.cast_list() {
        Ok(input_data) => input_data,
        Err(err) => bail!("Invalid input data #1 in NEURALNETWORKS.PREDICT: {}", err),
    };
    let mut i_data: Vec<f32> = Vec::new();
    for v in input_data {
        let value = match v.cast_float() {
            Ok(value) => value,
            Err(err) => bail!("Invalid data in input NEURALNETWORKS.PREDICT: {}", err),
        };
        i_data.push(value as f32);
    }
    return Ok(tensor::Tensor::single(i_data));
}

fn prepare_training_data(vm: &mut VM, conf: Value, name: String) -> Result<Vec<tensor::Tensor>, Error> {
    let training_data = helpers::conf::conf_get(vm, conf.clone(), name.to_string(), Value::list()).cast_list().unwrap();
    let mut data: Vec<tensor::Tensor> = Vec::new();
    for v in training_data {
        match v.dt {
            LIST => {
                match v.cast_list() {
                    Ok(row) => {
                        let mut x_row: Vec<f32> = Vec::new();
                        for value in row {
                            match value.cast_float() {
                                Ok(fvalue) => {
                                    x_row.push(fvalue as f32);
                                }
                                Err(err) => bail!("Tensor data conversion for {} returned: {}", &name, err),
                            }
                        }
                        data.push(tensor::Tensor::single(x_row));
                    }
                    Err(err) => bail!("Tensor data cazsting for {} returned: {}", &name, err),
                }
            }
            _ => bail!("Invalid data type for vector {}", &name),
        }
    }
    return Ok(data);
}

pub fn create_perceptron_nn(vm: &mut VM, name: String, conf: Value) -> Result<&mut VM, Error> {
    log::debug!("Create Perceptron named: {}", &name);
    let n_inputs = helpers::conf::conf_get(vm, conf.clone(), "Inputs".to_string(), Value::from_int(1)).cast_int().unwrap();
    let n_outputs = helpers::conf::conf_get(vm, conf.clone(), "Outputs".to_string(), Value::from_int(1)).cast_int().unwrap();
    let n_hidden = helpers::conf::conf_get(vm, conf.clone(), "Hidden".to_string(), Value::from_int(10)).cast_int().unwrap();
    let learning_rate = helpers::conf::conf_get(vm, conf.clone(), "LRate".to_string(), Value::from_float(0.1)).cast_float().unwrap();
    let decay = helpers::conf::conf_get(vm, conf.clone(), "Decay".to_string(), Value::from_float(0.01)).cast_float().unwrap();
    let batch = helpers::conf::conf_get(vm, conf.clone(), "Batch".to_string(), Value::from_int(4)).cast_int().unwrap();
    let epoch = helpers::conf::conf_get(vm, conf.clone(), "Epoch".to_string(), Value::from_int(500)).cast_int().unwrap();

    let x = match prepare_training_data(vm, conf.clone(), "X".to_string()) {
        Ok(x) => x,
        Err(err) => bail!("{}", err),
    };
    let y = match prepare_training_data(vm, conf.clone(), "Y".to_string()) {
        Ok(y) => y,
        Err(err) => bail!("{}", err),
    };

    let inputs: Vec<&tensor::Tensor> = x.iter().collect();
    let targets: Vec<&tensor::Tensor> = y.iter().collect();

    let mut perceptron_network = network::Network::new(tensor::Shape::Single(n_inputs as usize));
    perceptron_network.dense(n_hidden as usize, activation::Activation::ReLU, true, None);
    perceptron_network.dense(n_outputs as usize, activation::Activation::Sigmoid, false, None);
    perceptron_network.set_optimizer(optimizer::SGD::create(learning_rate as f32, Some(decay as f32)));
    perceptron_network.set_objective(objective::Objective::BinaryCrossEntropy, None);

    let (_epoch_loss, _val_loss, _val_acc) =
        perceptron_network.learn(&inputs, &targets, None, batch as usize, epoch as i32, None);

    let (val_loss, val_acc) = perceptron_network.validate(&inputs, &targets, 1e-1);
    log::debug!(
        "Final validation accuracy for {} : {:.2} % and loss: {:.5}",
        &name,
        val_acc * 100.0,
        val_loss
    );

    let mut ai = NN.lock().unwrap();
    ai.insert(name.to_string(), NNEntry::new_perceptron(perceptron_network));
    drop(ai);
    Ok(vm)
}

pub fn predict_perceptron_nn(vm: &mut VM, name: String, input: Value) -> Result<&mut VM, Error> {
    let input_data = match prepare_input_data_for_perceptron(vm, input) {
        Ok(input_data) => input_data,
        Err(err) => {
            bail!("{}", err);
        },
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
        match &nn.nn {
            NNVal::Perceptron(ai_network) => {
                let prediction = ai_network.predict(&input_data);
                match prediction.data {
                    tensor::Data::Single(perceptron_output) => {
                        let mut res = Value::list();
                        for v in perceptron_output {
                            res = res.push(Value::from_float(v as f64));
                        }
                        vm.stack.push(res);
                    }
                    _ => {
                        bail!("Perceptron returns an incalid data");
                    }
                }
            }
            _ => {
                drop(ai);
                bail!("NEURALNETWORKS.PREDICT not resolve network: {}", &name);
            }
        }
    } else {
        drop(ai);
        bail!("NEURALNETWORKS.PREDICT not found network: {}", &name);
    }
    drop(ai);
    Ok(vm)
}
