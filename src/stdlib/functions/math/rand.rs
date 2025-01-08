use rust_multistackvm::multistackvm::{VM};
use crate::stdlib::BUND;
use crate::stdlib::helpers;
use rust_dynamic::value::Value;
use easy_error::{Error};
use crate::cmd;
use rand_mt::Mt64;
use fastrand::u64;
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref RAND: Mutex<Mt64> = {
        let e: Mutex<Mt64> = Mutex::new(Mt64::new(u64(1..1000000000000)));
        e
    };
}

pub fn stdlib_math_random_int_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    let mut rnd = RAND.lock().unwrap();
    let val = rnd.next_u64();
    drop(rnd);
    vm.stack.push(Value::from_int((val as i64).abs()));
    Ok(vm)
}

pub fn init_stdlib(cli: &cmd::Cli) {
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };
    let rnd = RAND.lock().unwrap();
    log::debug!("Initialize INT random generator");
    drop(rnd);
    let _ = bc.vm.register_inline("math.random.int".to_string(), stdlib_math_random_int_inline);
}
