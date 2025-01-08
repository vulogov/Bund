extern crate log;
use crate::stdlib::functions::generators::{DIST, DEntry, DType, DVal};
use rust_multistackvm::multistackvm::{VM};
use crate::stdlib::helpers;
use rust_dynamic::value::Value;
use statrs::generate::InfiniteSawtooth;
use easy_error::{Error, bail};

impl DEntry {
    pub fn new_sawtooth(d: InfiniteSawtooth) -> Self {
        Self {
            id:     DType::Sawtooth,
            nn:     DVal::Sawtooth(d),
        }
    }
}



pub fn create_generator(vm: &mut VM, name: String, conf: Value) -> Result<&mut VM, Error> {
    log::debug!("Create sawtooth generator: {}", &name);
    let period = helpers::conf::conf_get(vm, conf.clone(), "Period".to_string(), Value::from_int(10)).cast_int().unwrap();
    let high = helpers::conf::conf_get(vm, conf.clone(), "High".to_string(), Value::from_float(1.0)).cast_float().unwrap();
    let low = helpers::conf::conf_get(vm, conf.clone(), "Low".to_string(), Value::from_float(0.0)).cast_float().unwrap();
    let delay = helpers::conf::conf_get(vm, conf.clone(), "Delay".to_string(), Value::from_int(1)).cast_int().unwrap();
    log::debug!("High: {} Low: {} Period: {} Delay: {}", high, low, period, delay);
    let n = InfiniteSawtooth::new(period, high, low, delay);
    let mut d = DIST.lock().unwrap();
    d.insert(name.to_string(), DEntry::new_sawtooth(n));
    drop(d);
    Ok(vm)
}

pub fn generator_sample(vm: &mut VM, name: String) -> Result<&mut VM, Error> {
    let g = DIST.lock().unwrap();
    if g.contains_key(&name) {
        let gen = match g.get(&name) {
            Some(gen) => gen,
            None => {
                drop(g);
                bail!("GENERATOR.SAMPLE not found generator: {}. It is not there.", &name);
            },
        };
        match &gen.nn {
            DVal::Sawtooth(mut n) => {
                let val = match n.next() {
                    Some(val) => val,
                    None => bail!("Failed to sample next sawtooth value"),
                };
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
    let g = DIST.lock().unwrap();
    if g.contains_key(&name) {
        let gen = match g.get(&name) {
            Some(gen) => gen,
            None => {
                drop(g);
                bail!("GENERATOR.SAMPLE* not found generator: {}. It is not there.", &name);
            },
        };
        match &gen.nn {
            DVal::Sawtooth(mut n) => {
                let mut res = Value::list();
                for _ in 0..n_elem {
                    let val = match n.next() {
                        Some(val) => val,
                        None => bail!("Failed to sample next sawtooth value"),
                    };
                    res = res.push(Value::from_float(val));
                }

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
