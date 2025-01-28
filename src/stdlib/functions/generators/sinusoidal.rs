extern crate log;
use crate::stdlib::functions::generators::{DIST, DEntry, DType, DVal};
use rust_multistackvm::multistackvm::{VM};
use crate::stdlib::helpers;
use rust_dynamic::value::Value;
use statrs::generate::InfiniteSinusoidal;
use easy_error::{Error, bail};

impl DEntry {
    pub fn new_sinusoidal(d: InfiniteSinusoidal) -> Self {
        Self {
            id:     DType::Sinusoidal,
            nn:     DVal::Sinusoidal(d),
            skip:   0,
        }
    }
}



pub fn create_generator(vm: &mut VM, name: String, conf: Value) -> Result<&mut VM, Error> {
    log::debug!("Create sinusoidal generator: {}", &name);

    let sampling = helpers::conf::conf_get(vm, conf.clone(), "Sampling".to_string(), Value::from_float(10.0)).cast_float().unwrap();
    let freq = helpers::conf::conf_get(vm, conf.clone(), "Freq".to_string(), Value::from_float(2.0)).cast_float().unwrap();
    let amplitude = helpers::conf::conf_get(vm, conf.clone(), "Amplitude".to_string(), Value::from_float(10.0)).cast_float().unwrap();
    let phase = helpers::conf::conf_get(vm, conf.clone(), "Phase".to_string(), Value::from_float(1.0)).cast_float().unwrap();
    let mean = helpers::conf::conf_get(vm, conf.clone(), "Mean".to_string(), Value::from_float(5.0)).cast_float().unwrap();
    let delay = helpers::conf::conf_get(vm, conf.clone(), "Delay".to_string(), Value::from_int(1)).cast_int().unwrap();
    log::debug!("Sampling: {} Freq: {} Amplitude: {} Phase: {} Delay {}", sampling, freq, amplitude, phase, delay);

    let n = InfiniteSinusoidal::new(sampling, freq, amplitude, mean, phase, delay);
    let mut d = DIST.lock().unwrap();
    d.insert(name.to_string(), DEntry::new_sinusoidal(n));
    drop(d);
    Ok(vm)
}

pub fn generator_sample(vm: &mut VM, name: String) -> Result<&mut VM, Error> {
    let mut g = DIST.lock().unwrap();
    if g.contains_key(&name) {
        let gen = match g.get_mut(&name) {
            Some(gen) => gen,
            None => {
                drop(g);
                bail!("GENERATOR.SAMPLE not found generator: {}. It is not there.", &name);
            },
        };
        match &gen.nn {
            DVal::Sinusoidal(n) => {
                let val = match n.skip(gen.skip as usize).next() {
                    Some(val) => val,
                    None => bail!("Failed to sample next periodic value"),
                };
                gen.skip += 1;
                vm.stack.push(Value::from_float(val));
            }
            _ => {
                drop(g);
                bail!("GENERATOR.SAMPLE not resolve generator: {}", &name);
            }
        }
    } else {
        drop(g);
        bail!("GENERATOR.SAMPLE not found generator: {}", &name);
    }
    drop(g);
    Ok(vm)
}

pub fn generator_n_sample(vm: &mut VM, name: String, n_elem: i64) -> Result<&mut VM, Error> {
    let mut g = DIST.lock().unwrap();
    if g.contains_key(&name) {
        let gen = match g.get_mut(&name) {
            Some(gen) => gen,
            None => {
                drop(g);
                bail!("GENERATOR.SAMPLE* not found generator: {}. It is not there.", &name);
            },
        };
        match &gen.nn {
            DVal::Sinusoidal(n) => {
                let mut res = Value::list();
                let mut i = n.skip(gen.skip as usize);
                for _ in 0..n_elem {
                    let val = match i.next() {
                        Some(val) => val,
                        None => bail!("Failed to sample next sinusoidal value"),
                    };
                    gen.skip += 1;
                    res = res.push(Value::from_float(val));
                }
                gen.skip += 1;
                vm.stack.push(res);
            }
            _ => {
                drop(g);
                bail!("GENERATOR.SAMPLE* not resolve generator: {}", &name);
            }
        }
    } else {
        drop(g);
        bail!("GENERATOR.SAMPLE* not found generator: {}", &name);
    }
    drop(g);
    Ok(vm)
}
