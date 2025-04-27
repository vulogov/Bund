extern crate log;
use crate::stdlib::functions::ai::{NN, NNEntry, NNType, NNVal};
use rust_multistackvm::multistackvm::{VM};
use rust_dynamic::value::Value;
use rustrict::{Censor, Type};
use easy_error::{Error, bail};

impl NNEntry {
    pub fn new_profanity_classifier() -> Self {
        Self {
            id:     NNType::Profanity,
            nn:     NNVal::Null,
        }
    }
}


pub fn create_profanity_classifier(vm: &mut VM, name: String, _conf: Value) -> Result<&mut VM, Error> {
    log::debug!("Create Profanity classifier named: {}", &name);

    let mut ai = NN.lock().unwrap();
    ai.insert(name.to_string(), NNEntry::new_profanity_classifier());
    drop(ai);
    Ok(vm)
}

pub fn classify_profanity_classifier(vm: &mut VM, name: String, input: Value) -> Result<&mut VM, Error> {
    let input_text = match input.cast_string() {
        Ok(input_text) => input_text,
        Err(err) => bail!("PROFANITY classify returns error on input: {}", err),
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
        match &nn.id {
            NNType::Profanity => {
                let res = Censor::from_str(&input_text)
                        .with_censor_first_character_threshold(Type::OFFENSIVE & Type::SEVERE)
                        .with_ignore_false_positives(false)
                        .analyze();
                if res.is(Type::INAPPROPRIATE | Type::PROFANE | Type::SEVERE | Type::SEXUAL | Type::SPAM) {
                    vm.stack.push(Value::from_bool(true));
                } else {
                    vm.stack.push(Value::from_bool(false));
                }
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
