extern crate log;
use rust_multistackvm::multistackvm::{VM};
use crate::stdlib::helpers;
use rust_dynamic::value::Value;
use crate::stdlib::functions::generators;
use crate::stdlib::functions::generators::{DIST, DType};
use easy_error::{Error, bail};

pub fn stdlib_generator_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 2 {
        bail!("Stack is too shallow for inline GENERATOR");
    }
    match vm.stack.pull() {
        Some(conf) => {
            let name_value = match vm.stack.pull() {
                Some(name_value) => name_value,
                None => bail!("GENERATOR returns: NO DATA #2")
            };
            let name: String = match name_value.cast_string() {
                Ok(name) => name,
                Err(err) => bail!("GENERATOR name casting returns: {}", err),
            };
            let g_type = helpers::conf::conf_get(vm, conf.clone(), "type".to_string(), Value::from_string("unknown"));
            let res = match g_type.cast_string().unwrap().as_str() {
                "normal" => generators::normal::create_generator(vm, name, conf),
                "uniform" => generators::uniform::create_generator(vm, name, conf),
                "lognormal" => generators::lognormal::create_generator(vm, name, conf),
                _ => bail!("Unknown GENERATOR type: {}", &g_type),
            };
            return res;
        }
        None => {
            bail!("GENERATOR returns: NO DATA #1");
        }
    }
}

pub fn stdlib_generator_sample_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline GENERATOR.SAMPLE");
    }
    match vm.stack.pull() {
        Some(name_value) => {
            let name = match name_value.cast_string() {
                Ok(name) => name,
                Err(err) => bail!("GENERATOR.SAMPLE casting a name returns: {}", err),
            };
            let g = DIST.lock().unwrap();
            if g.contains_key(&name) {
                let gen = match g.get(&name) {
                    Some(gen) => gen,
                    None => {
                        drop(g);
                        bail!("GENERATOR.SAMPLE not found generator: {}. It is not there.", &name);
                    },
                };
                match gen.id {
                    DType::Normal => {
                        drop(g);
                        return generators::normal::generator_sample(vm, name);
                    }
                    DType::Uniform => {
                        drop(g);
                        return generators::uniform::generator_sample(vm, name);
                    }
                    DType::LogNormal => {
                        drop(g);
                        return generators::lognormal::generator_sample(vm, name);
                    }
                    _ => {
                        drop(g);
                        bail!("GENERATOR.SAMPLE generator having an invalid type: {}", &name);
                    }
                }
            } else {
                drop(g);
                bail!("GENERATOR.SAMPLE not found generator: {}", &name);
            }
        }
        None => {
            bail!("GENERATOR.SAMPLE returns: NO DATA #1");
        }
    }
}

pub fn stdlib_generator_n_sample_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 2 {
        bail!("Stack is too shallow for inline GENERATOR.SAMPLE*");
    }
    let n_value = match vm.stack.pull() {
        Some(n_value) => n_value,
        None => bail!("GENERATOR.SAMPLE returns: NO DATA #2"),
    };
    let n = match n_value.cast_int() {
        Ok(name) => name,
        Err(err) => bail!("GENERATOR.SAMPLE* casting a name returns: {}", err),
    };
    match vm.stack.pull() {
        Some(name_value) => {
            let name = match name_value.cast_string() {
                Ok(name) => name,
                Err(err) => bail!("GENERATOR.SAMPLE* casting a name returns: {}", err),
            };
            let g = DIST.lock().unwrap();
            if g.contains_key(&name) {
                let gen = match g.get(&name) {
                    Some(gen) => gen,
                    None => {
                        drop(g);
                        bail!("GENERATOR.SAMPLE* not found generator: {}. It is not there.", &name);
                    },
                };
                match gen.id {
                    DType::Normal => {
                        drop(g);
                        return generators::normal::generator_n_sample(vm, name, n);
                    }
                    DType::Uniform => {
                        drop(g);
                        return generators::uniform::generator_n_sample(vm, name, n);
                    }
                    DType::LogNormal => {
                        drop(g);
                        return generators::lognormal::generator_n_sample(vm, name, n);
                    }
                    _ => {
                        drop(g);
                        bail!("GENERATOR.SAMPLE* generator having an invalid type: {}", &name);
                    }
                }
            } else {
                drop(g);
                bail!("GENERATOR.SAMPLE* not found generator: {}", &name);
            }
        }
        None => {
            bail!("GENERATOR.SAMPLE returns: NO DATA #1");
        }
    }
}
