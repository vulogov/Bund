extern crate log;
use crate::cmd;
use crate::stdlib::helpers;
use std::sync::Mutex;
use std::collections::btree_map::BTreeMap;
use lazy_static::lazy_static;
use crate::stdlib::BUND;

use statrs::distribution::{Normal};

pub mod generator;
pub mod normal;

#[derive(Clone, Debug)]
pub enum DType {
    Normal,
    Binomial,
}

pub enum DVal {
    Null,
    Normal(Normal),
}


pub struct DEntry {
    pub id:     DType,
    pub nn:     DVal,
}

lazy_static! {
    static ref DIST: Mutex<BTreeMap<String, DEntry>> = {
        let e: Mutex<BTreeMap<String, DEntry>> = Mutex::new(BTreeMap::new());
        e
    };
}


pub fn init_stdlib(cli: &cmd::Cli) {
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };

    let _ = bc.vm.register_inline("generator".to_string(), generator::stdlib_generator_inline);
    let _ = bc.vm.register_inline("generator.sample".to_string(), generator::stdlib_generator_sample_inline);
    let _ = bc.vm.register_inline("generator.sample*".to_string(), generator::stdlib_generator_n_sample_inline);
    drop(bc);
}
