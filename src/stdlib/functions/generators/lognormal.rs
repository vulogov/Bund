extern crate log;
use crate::stdlib::functions::generators::{DIST, DEntry, DType, DVal};
use rust_multistackvm::multistackvm::{VM};
use crate::stdlib::helpers;
use rust_dynamic::value::Value;
use rand::distributions::Distribution;
use statrs::distribution::{LogNormal};
use easy_error::{Error, bail};

impl DEntry {
    pub fn new_lognormal(d: LogNormal) -> Self {
        Self {
            id:     DType::LogNormal,
            nn:     DVal::LogNormal(d),
            skip:   0,
        }
    }
}



pub fn create_generator(vm: &mut VM, name: String, conf: Value) -> Result<&mut VM, Error> {
    log::debug!("Create lognormal distribution generator: {}", &name);
    let loc = helpers::conf::conf_get(vm, conf.clone(), "Location".to_string(), Value::from_float(0.0)).cast_float().unwrap();
    let scale = helpers::conf::conf_get(vm, conf.clone(), "Scale".to_string(), Value::from_float(1.0)).cast_float().unwrap();
    log::debug!("Location: {} Scale: {}", loc, scale);
    let n = match LogNormal::new(loc, scale) {
        Ok(n) => n,
        Err(err) => bail!("GENERATOR returned: {}", err),
    };
    let mut d = DIST.lock().unwrap();
    d.insert(name.to_string(), DEntry::new_lognormal(n));
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
            DVal::LogNormal(n) => {
                let mut r = rand::thread_rng();
                let val = n.sample(&mut r);
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
            DVal::LogNormal(n) => {
                let mut r = rand::thread_rng();
                let mut res = Value::list();
                for _ in 0..n_elem {
                    let val = n.sample(&mut r);
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
