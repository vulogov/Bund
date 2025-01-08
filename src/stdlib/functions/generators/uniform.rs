extern crate log;
use crate::stdlib::functions::generators::{DIST, DEntry, DType, DVal};
use rust_multistackvm::multistackvm::{VM};
use crate::stdlib::helpers;
use rust_dynamic::value::Value;
use rand::distributions::{Distribution};
use statrs::distribution::{Uniform};
use easy_error::{Error, bail};

impl DEntry {
    pub fn new_uniform(d: Uniform) -> Self {
        Self {
            id:     DType::Uniform,
            nn:     DVal::Uniform(d),
        }
    }
}



pub fn create_generator(vm: &mut VM, name: String, conf: Value) -> Result<&mut VM, Error> {
    log::debug!("Create uniform distribution generator: {}", &name);
    let y_max = helpers::conf::conf_get(vm, conf.clone(), "Max".to_string(), Value::from_float(1.0)).cast_float().unwrap();
    let y_min = helpers::conf::conf_get(vm, conf.clone(), "Min".to_string(), Value::from_float(0.0)).cast_float().unwrap();
    log::debug!("Min: {} Max: {}", y_min, y_max);
    let n = match Uniform::new(y_min, y_max) {
        Ok(n) => n,
        Err(err) => bail!("GENERATOR returned: {}", err),
    };
    let mut d = DIST.lock().unwrap();
    d.insert(name.to_string(), DEntry::new_uniform(n));
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
            DVal::Normal(n) => {
                let mut r = rand::thread_rng();
                let val = n.sample(&mut r);
                vm.stack.push(Value::from_float(val));
            }
            DVal::Uniform(n) => {
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
            DVal::Normal(n) => {
                let mut r = rand::thread_rng();
                let mut res = Value::list();
                for _ in 0..n_elem {
                    let val = n.sample(&mut r);
                    res = res.push(Value::from_float(val));
                }

                vm.stack.push(res);
            }
            DVal::Uniform(n) => {
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
