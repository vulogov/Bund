extern crate log;
use rust_multistackvm::multistackvm::{VM};
use crate::stdlib::helpers;
use rust_dynamic::value::Value;
use crate::stdlib::functions::ai::naivebayes;
use crate::stdlib::functions::ai::linguistic;
use easy_error::{Error, bail};

pub fn stdlib_classifier_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 2 {
        bail!("Stack is too shallow for inline CLASSIFIERS");
    }
    match vm.stack.pull() {
        Some(conf) => {
            let name_value = match vm.stack.pull() {
                Some(name_value) => name_value,
                None => bail!("CLASSIFIERS returns: NO DATA #2")
            };
            let name: String = match name_value.cast_string() {
                Ok(name) => name,
                Err(err) => bail!("CLASSIFIERS name casting returns: {}", err),
            };
            let nn_type = helpers::conf::conf_get(vm, conf.clone(), "type".to_string(), Value::from_string("seq.ascending"));
            let res = match nn_type.cast_string().unwrap().as_str() {
                "naivebayes" => naivebayes::create_naivebayes_classifier(vm, name, conf),
                "linguistic" => linguistic::create_linguistic_classifier(vm, name, conf),
                _ => bail!("Unknown CLASSIFIER type: {}", &nn_type),
            };
            return res;
        }
        None => {
            bail!("CLASSIFIERS returns: NO DATA #1");
        }
    }
}
