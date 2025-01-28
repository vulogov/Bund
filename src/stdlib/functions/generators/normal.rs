extern crate log;
use crate::stdlib::functions::generators::{DIST, DEntry, DType, DVal};
use rust_multistackvm::multistackvm::{VM};
use crate::stdlib::helpers;
use rust_dynamic::value::Value;
use rand::distributions::{Distribution};
use statrs::distribution::{Normal};
use easy_error::{Error, bail};

impl DEntry {
    pub fn new_normal(d: Normal) -> Self {
        Self {
            id:     DType::Normal,
            nn:     DVal::Normal(d),
            skip:   0,
        }
    }
}



pub fn create_generator(vm: &mut VM, name: String, conf: Value) -> Result<&mut VM, Error> {
    log::debug!("Create gaussian distribution generator: {}", &name);
    let x_mean = helpers::conf::conf_get(vm, conf.clone(), "Mean".to_string(), Value::from_float(0.0)).cast_float().unwrap();
    let y_deviation = helpers::conf::conf_get(vm, conf.clone(), "Deviation".to_string(), Value::from_float(1.0)).cast_float().unwrap();
    log::debug!("Mean value: {} STD: {}", x_mean, y_deviation);
    let n = match Normal::new(x_mean, y_deviation) {
        Ok(n) => n,
        Err(err) => bail!("GENERATOR returned: {}", err),
    };
    let mut d = DIST.lock().unwrap();
    d.insert(name.to_string(), DEntry::new_normal(n));
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
